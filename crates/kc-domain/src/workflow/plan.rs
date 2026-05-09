use std::collections::BTreeSet;

use crate::{
    ConfirmationId, ConfirmationKind, DomainError, ExecutionId, ExecutionMode, LogAttribution,
    OperationId, OperationTarget, PlanId, PlanItemId, PlanItemKind, WorkflowKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationGate {
    pub id: ConfirmationId,
    pub kind: ConfirmationKind,
    pub message: String,
    pub plan_item_id: Option<PlanItemId>,
}

impl ConfirmationGate {
    pub fn for_plan_item(
        id: ConfirmationId,
        kind: ConfirmationKind,
        message: impl Into<String>,
        plan_item_id: PlanItemId,
    ) -> Self {
        Self {
            id,
            kind,
            message: message.into(),
            plan_item_id: Some(plan_item_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanItem {
    pub id: PlanItemId,
    pub kind: PlanItemKind,
    pub summary: String,
    pub target: OperationTarget,
    pub destructive: bool,
    pub confirmation: Option<ConfirmationKind>,
}

impl PlanItem {
    pub fn new(
        id: PlanItemId,
        kind: PlanItemKind,
        summary: impl Into<String>,
        target: OperationTarget,
    ) -> Self {
        Self {
            id,
            kind,
            summary: summary.into(),
            target,
            destructive: false,
            confirmation: None,
        }
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn requires_confirmation(mut self, kind: ConfirmationKind) -> Self {
        self.confirmation = Some(kind);
        self
    }

    pub fn log_attribution(
        &self,
        plan_id: PlanId,
        execution_id: ExecutionId,
        operation_id: OperationId,
    ) -> LogAttribution {
        LogAttribution {
            plan_id,
            plan_item_id: self.id,
            execution_id,
            operation_id,
            target: self.target.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowPlan {
    pub id: PlanId,
    pub kind: WorkflowKind,
    pub mode: ExecutionMode,
    pub items: Vec<PlanItem>,
    pub confirmations: Vec<ConfirmationGate>,
}

impl WorkflowPlan {
    pub fn new(
        id: PlanId,
        kind: WorkflowKind,
        mode: ExecutionMode,
        items: Vec<PlanItem>,
    ) -> Result<Self, DomainError> {
        let mut seen_item_ids = BTreeSet::new();
        let mut seen_confirmation_ids = BTreeSet::new();
        let mut confirmations = Vec::new();
        for item in &items {
            if !seen_item_ids.insert(item.id) {
                return Err(DomainError::DuplicatePlanItem(item.id));
            }

            if let Some(kind) = item.confirmation {
                let confirmation = ConfirmationGate::for_plan_item(
                    ConfirmationId::new(item.id.value()),
                    kind,
                    item.summary.clone(),
                    item.id,
                );
                if !seen_confirmation_ids.insert(confirmation.id) {
                    return Err(DomainError::DuplicateConfirmation(confirmation.id));
                }
                confirmations.push(confirmation);
            }
        }

        Ok(Self {
            id,
            kind,
            mode,
            items,
            confirmations,
        })
    }

    pub fn with_confirmation(
        mut self,
        confirmation: ConfirmationGate,
    ) -> Result<Self, DomainError> {
        if self
            .confirmations
            .iter()
            .any(|existing| existing.id == confirmation.id)
        {
            return Err(DomainError::DuplicateConfirmation(confirmation.id));
        }
        self.confirmations.push(confirmation);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfirmationGate, PlanItem, WorkflowPlan};
    use crate::{
        ConfirmationId, ConfirmationKind, DomainError, ExecutionMode, OperationTarget, PlanId,
        PlanItemId, PlanItemKind, WorkflowKind,
    };

    #[test]
    fn workflow_plan_rejects_duplicate_item_ids() {
        let error = WorkflowPlan::new(
            PlanId::new(8),
            WorkflowKind::Install,
            ExecutionMode::Guarded,
            vec![
                PlanItem::new(
                    PlanItemId::new(1),
                    PlanItemKind::Read,
                    "Inspect device",
                    OperationTarget::Logical("device".to_string()),
                ),
                PlanItem::new(
                    PlanItemId::new(1),
                    PlanItemKind::Write,
                    "Write payload",
                    OperationTarget::Logical("payload".to_string()),
                ),
            ],
        )
        .unwrap_err();

        assert_eq!(error, DomainError::DuplicatePlanItem(PlanItemId::new(1)));
    }

    #[test]
    fn workflow_plan_rejects_duplicate_confirmation_ids() {
        let plan = WorkflowPlan::new(
            PlanId::new(10),
            WorkflowKind::Install,
            ExecutionMode::Guarded,
            vec![PlanItem::new(
                PlanItemId::new(1),
                PlanItemKind::Write,
                "Write payload",
                OperationTarget::Logical("payload".to_string()),
            )
            .requires_confirmation(ConfirmationKind::ExternalDeviceMutation)],
        )
        .unwrap();

        let error = plan
            .with_confirmation(ConfirmationGate {
                id: ConfirmationId::new(1),
                kind: ConfirmationKind::ManualPrecondition,
                message: "Extra gate".to_string(),
                plan_item_id: None,
            })
            .unwrap_err();

        assert_eq!(
            error,
            DomainError::DuplicateConfirmation(ConfirmationId::new(1))
        );
    }
}
