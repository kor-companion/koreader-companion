use std::error::Error;
use std::fmt;

use crate::{
    workflow::{ConfirmationId, PlanItemId, WorkflowPhase},
    Capability, SafetyViolation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    MissingCapabilities {
        subject: String,
        missing: Vec<Capability>,
    },
    InvalidWorkflowTransition {
        from: WorkflowPhase,
        to: WorkflowPhase,
    },
    DuplicatePlanItem(PlanItemId),
    DuplicateConfirmation(ConfirmationId),
    UnknownConfirmation(ConfirmationId),
    UnknownPlanItem(PlanItemId),
    Safety(SafetyViolation),
    Validation(String),
    Unsupported(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCapabilities { subject, missing } => {
                write!(f, "{subject} is missing capabilities: {missing:?}")
            }
            Self::InvalidWorkflowTransition { from, to } => {
                write!(f, "invalid workflow transition from {from:?} to {to:?}")
            }
            Self::DuplicatePlanItem(id) => write!(f, "duplicate plan item id {}", id.value()),
            Self::DuplicateConfirmation(id) => {
                write!(f, "duplicate confirmation id {}", id.value())
            }
            Self::UnknownConfirmation(id) => write!(f, "unknown confirmation id {}", id.value()),
            Self::UnknownPlanItem(id) => write!(f, "unknown plan item id {}", id.value()),
            Self::Safety(error) => write!(f, "{error}"),
            Self::Validation(message) | Self::Unsupported(message) => write!(f, "{message}"),
        }
    }
}

impl Error for DomainError {}

impl From<SafetyViolation> for DomainError {
    fn from(value: SafetyViolation) -> Self {
        Self::Safety(value)
    }
}
