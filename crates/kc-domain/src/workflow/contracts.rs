use crate::{
    DomainError, VerificationReport, WorkflowPlan, WorkflowProgressUpdate, WorkflowSnapshot,
};

pub trait WorkflowPlanner {
    fn kind(&self) -> crate::WorkflowKind;
    fn plan(&self) -> Result<WorkflowPlan, DomainError>;
}

pub trait Workflow: WorkflowPlanner {}

impl<T> Workflow for T where T: WorkflowPlanner + ?Sized {}

pub trait WorkflowProgressSink {
    fn record_progress(&mut self, update: &WorkflowProgressUpdate) -> Result<(), DomainError>;
}

pub trait CancellationSignal {
    fn is_cancelled(&self) -> bool;

    fn reason(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowExecutionResult {
    pub snapshot: WorkflowSnapshot,
    pub verification: Option<VerificationReport>,
}

pub trait WorkflowExecutor {
    fn execute(
        &self,
        plan: &WorkflowPlan,
        progress: &mut dyn WorkflowProgressSink,
        cancellation: &dyn CancellationSignal,
    ) -> Result<WorkflowExecutionResult, DomainError>;
}

pub trait WorkflowVerifier {
    fn verify(&self, snapshot: &WorkflowSnapshot) -> Result<VerificationReport, DomainError>;
}

pub trait WorkflowDefinition: WorkflowPlanner + WorkflowExecutor + WorkflowVerifier {}

impl<T> WorkflowDefinition for T where T: WorkflowPlanner + WorkflowExecutor + WorkflowVerifier {}
