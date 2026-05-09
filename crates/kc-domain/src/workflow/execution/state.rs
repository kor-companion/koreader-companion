use std::collections::BTreeMap;

use crate::{
    workflow::{ConfirmationId, PlanItemId, WorkflowPhase},
    ConfirmationGate, WorkflowPlan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowExecution {
    pub(super) plan: WorkflowPlan,
    pub(super) phase: WorkflowPhase,
    pub(super) pending_confirmations: BTreeMap<ConfirmationId, ConfirmationGate>,
    pub(super) completed_items: Vec<PlanItemId>,
    pub(super) active_item: Option<PlanItemId>,
    pub(super) failure_message: Option<String>,
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
}
