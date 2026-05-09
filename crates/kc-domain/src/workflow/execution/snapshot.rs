use crate::WorkflowSnapshot;

use super::WorkflowExecution;

impl WorkflowExecution {
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
}
