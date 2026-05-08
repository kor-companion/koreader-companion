use std::collections::BTreeMap;

use crate::{
    workflow::{ConfirmationId, PlanItemId, WorkflowPhase},
    ConfirmationGate, DomainError, WorkflowPlan, WorkflowSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowExecution {
    plan: WorkflowPlan,
    phase: WorkflowPhase,
    pending_confirmations: BTreeMap<ConfirmationId, ConfirmationGate>,
    completed_items: Vec<PlanItemId>,
    active_item: Option<PlanItemId>,
    failure_message: Option<String>,
}

impl WorkflowExecution {
    pub fn new(plan: WorkflowPlan) -> Self {
        let pending_confirmations = plan
            .confirmations
            .iter()
            .cloned()
            .map(|confirmation| (confirmation.id, confirmation))
            .collect::<BTreeMap<_, _>>();

        let phase = if pending_confirmations.is_empty() {
            WorkflowPhase::Ready
        } else {
            WorkflowPhase::AwaitingConfirmation
        };

        Self {
            plan,
            phase,
            pending_confirmations,
            completed_items: Vec::new(),
            active_item: None,
            failure_message: None,
        }
    }

    pub fn plan(&self) -> &WorkflowPlan {
        &self.plan
    }

    pub fn phase(&self) -> WorkflowPhase {
        self.phase
    }

    pub fn snapshot(&self) -> WorkflowSnapshot {
        WorkflowSnapshot {
            plan_id: self.plan.id,
            kind: self.plan.kind,
            mode: self.plan.mode,
            phase: self.phase,
            total_items: self.plan.items.len(),
            completed_items: self.completed_items.len(),
            active_item: self.active_item,
            pending_confirmations: self.pending_confirmations.values().cloned().collect(),
            failure_message: self.failure_message.clone(),
        }
    }

    pub fn approve_confirmation(&mut self, id: ConfirmationId) -> Result<(), DomainError> {
        if self.phase != WorkflowPhase::AwaitingConfirmation {
            return Err(DomainError::InvalidWorkflowTransition {
                from: self.phase,
                to: WorkflowPhase::Ready,
            });
        }

        if self.pending_confirmations.remove(&id).is_none() {
            return Err(DomainError::UnknownConfirmation(id));
        }

        if self.pending_confirmations.is_empty() {
            self.phase = WorkflowPhase::Ready;
        }

        Ok(())
    }

    pub fn reject_confirmation(
        &mut self,
        id: ConfirmationId,
        reason: impl Into<String>,
    ) -> Result<(), DomainError> {
        if self.pending_confirmations.remove(&id).is_none() {
            return Err(DomainError::UnknownConfirmation(id));
        }

        self.phase = WorkflowPhase::Cancelled;
        self.failure_message = Some(reason.into());
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), DomainError> {
        if self.phase != WorkflowPhase::Ready {
            return Err(DomainError::InvalidWorkflowTransition {
                from: self.phase,
                to: WorkflowPhase::Running,
            });
        }

        self.phase = WorkflowPhase::Running;
        Ok(())
    }

    pub fn begin_item(&mut self, item_id: PlanItemId) -> Result<(), DomainError> {
        if self.phase != WorkflowPhase::Running {
            return Err(DomainError::InvalidWorkflowTransition {
                from: self.phase,
                to: WorkflowPhase::Running,
            });
        }

        if self.active_item.is_some() || self.completed_items.contains(&item_id) {
            return Err(DomainError::Validation(format!(
                "plan item {} is not available to start",
                item_id.value()
            )));
        }

        if !self.plan.items.iter().any(|item| item.id == item_id) {
            return Err(DomainError::UnknownPlanItem(item_id));
        }

        self.active_item = Some(item_id);
        Ok(())
    }

    pub fn complete_active_item(&mut self) -> Result<PlanItemId, DomainError> {
        if self.phase != WorkflowPhase::Running {
            return Err(DomainError::InvalidWorkflowTransition {
                from: self.phase,
                to: WorkflowPhase::Running,
            });
        }

        let item_id = self
            .active_item
            .take()
            .ok_or_else(|| DomainError::Validation("no active plan item".to_string()))?;

        self.completed_items.push(item_id);
        Ok(item_id)
    }

    pub fn finish(&mut self) -> Result<(), DomainError> {
        if self.phase != WorkflowPhase::Running {
            return Err(DomainError::InvalidWorkflowTransition {
                from: self.phase,
                to: WorkflowPhase::Succeeded,
            });
        }

        if self.active_item.is_some() {
            return Err(DomainError::Validation(
                "cannot finish while a plan item is active".to_string(),
            ));
        }

        if self.completed_items.len() != self.plan.items.len() {
            return Err(DomainError::Validation(
                "cannot finish before all plan items complete".to_string(),
            ));
        }

        self.phase = WorkflowPhase::Succeeded;
        Ok(())
    }

    pub fn fail(&mut self, message: impl Into<String>) -> Result<(), DomainError> {
        ensure_terminal_transition_allowed(self.phase, WorkflowPhase::Failed)?;
        self.phase = WorkflowPhase::Failed;
        self.failure_message = Some(message.into());
        Ok(())
    }

    pub fn cancel(&mut self, message: impl Into<String>) -> Result<(), DomainError> {
        ensure_terminal_transition_allowed(self.phase, WorkflowPhase::Cancelled)?;
        self.phase = WorkflowPhase::Cancelled;
        self.failure_message = Some(message.into());
        Ok(())
    }
}

fn ensure_terminal_transition_allowed(
    from: WorkflowPhase,
    to: WorkflowPhase,
) -> Result<(), DomainError> {
    if matches!(
        from,
        WorkflowPhase::Succeeded | WorkflowPhase::Failed | WorkflowPhase::Cancelled
    ) {
        return Err(DomainError::InvalidWorkflowTransition { from, to });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::WorkflowExecution;
    use crate::{
        ConfirmationId, ConfirmationKind, DomainError, ExecutionMode, OperationTarget, PlanId,
        PlanItem, PlanItemId, PlanItemKind, WorkflowKind, WorkflowPhase, WorkflowPlan,
    };

    #[test]
    fn workflow_execution_advances_through_guarded_states() {
        let plan = WorkflowPlan::new(
            PlanId::new(7),
            WorkflowKind::Install,
            ExecutionMode::Guarded,
            vec![
                PlanItem::new(
                    PlanItemId::new(1),
                    PlanItemKind::Backup,
                    "Back up launcher state",
                    OperationTarget::DevicePath(PathBuf::from("/.kobo")),
                ),
                PlanItem::new(
                    PlanItemId::new(2),
                    PlanItemKind::Write,
                    "Write KOReader payload",
                    OperationTarget::DevicePath(PathBuf::from("/.adds/koreader")),
                )
                .destructive(true)
                .requires_confirmation(ConfirmationKind::ExternalDeviceMutation),
            ],
        )
        .unwrap();

        let mut execution = WorkflowExecution::new(plan);
        assert_eq!(execution.phase(), WorkflowPhase::AwaitingConfirmation);

        execution
            .approve_confirmation(ConfirmationId::new(2))
            .unwrap();
        assert_eq!(execution.phase(), WorkflowPhase::Ready);

        execution.start().unwrap();
        execution.begin_item(PlanItemId::new(1)).unwrap();
        assert_eq!(
            execution.complete_active_item().unwrap(),
            PlanItemId::new(1)
        );
        execution.begin_item(PlanItemId::new(2)).unwrap();
        assert_eq!(
            execution.complete_active_item().unwrap(),
            PlanItemId::new(2)
        );
        execution.finish().unwrap();

        let snapshot = execution.snapshot();
        assert_eq!(snapshot.phase, WorkflowPhase::Succeeded);
        assert_eq!(snapshot.completed_items, 2);
    }

    #[test]
    fn terminal_workflow_phases_cannot_be_overwritten() {
        let plan = WorkflowPlan::new(
            PlanId::new(9),
            WorkflowKind::Verify,
            ExecutionMode::DryRun,
            vec![PlanItem::new(
                PlanItemId::new(1),
                PlanItemKind::Validate,
                "Verify staged payload",
                OperationTarget::Logical("payload".to_string()),
            )],
        )
        .unwrap();

        let mut execution = WorkflowExecution::new(plan);
        execution.start().unwrap();
        execution.begin_item(PlanItemId::new(1)).unwrap();
        execution.complete_active_item().unwrap();
        execution.finish().unwrap();

        let error = execution.fail("should stay succeeded").unwrap_err();
        assert_eq!(
            error,
            DomainError::InvalidWorkflowTransition {
                from: WorkflowPhase::Succeeded,
                to: WorkflowPhase::Failed,
            }
        );
    }
}
