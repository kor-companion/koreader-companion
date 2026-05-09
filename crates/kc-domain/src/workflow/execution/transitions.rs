use crate::{
    workflow::{ConfirmationId, PlanItemId, WorkflowPhase},
    DomainError,
};

use super::WorkflowExecution;

impl WorkflowExecution {
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
