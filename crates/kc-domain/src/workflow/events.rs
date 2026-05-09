use crate::{ConfirmationGate, ExecutionMode, PlanId, PlanItemId, WorkflowKind, WorkflowPhase};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSnapshot {
    pub plan_id: PlanId,
    pub kind: WorkflowKind,
    pub mode: ExecutionMode,
    pub phase: WorkflowPhase,
    pub total_items: usize,
    pub completed_items: usize,
    pub active_item: Option<PlanItemId>,
    pub pending_confirmations: Vec<ConfirmationGate>,
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowProgressUpdate {
    pub plan_id: PlanId,
    pub phase: WorkflowPhase,
    pub completed_items: usize,
    pub total_items: usize,
    pub active_item: Option<PlanItemId>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    Pending,
    Passed,
    Warning,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationItem {
    pub subject: String,
    pub status: VerificationStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    pub status: VerificationStatus,
    pub items: Vec<VerificationItem>,
}

impl VerificationReport {
    pub fn passed(subject: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Passed,
            items: vec![VerificationItem {
                subject: subject.into(),
                status: VerificationStatus::Passed,
                message: None,
            }],
        }
    }

    pub fn warning(subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Warning,
            items: vec![VerificationItem {
                subject: subject.into(),
                status: VerificationStatus::Warning,
                message: Some(message.into()),
            }],
        }
    }

    pub fn failed(subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Failed,
            items: vec![VerificationItem {
                subject: subject.into(),
                status: VerificationStatus::Failed,
                message: Some(message.into()),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    WorkflowPlanned(WorkflowSnapshot),
    ConfirmationRequired(ConfirmationGate, WorkflowSnapshot),
    PhaseChanged(WorkflowPhase, WorkflowSnapshot),
    ProgressUpdated(WorkflowProgressUpdate, WorkflowSnapshot),
    VerificationReported(VerificationReport, WorkflowSnapshot),
    PlanItemStarted(PlanItemId, WorkflowSnapshot),
    PlanItemCompleted(PlanItemId, WorkflowSnapshot),
    WorkflowFinished(WorkflowSnapshot),
}
