#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowKind {
    Discover,
    Install,
    Backup,
    Restore,
    Verify,
    Eject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    DryRun,
    Guarded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanItemKind {
    Read,
    Write,
    Backup,
    Validate,
    Sync,
    Eject,
    Notify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationKind {
    DestructiveWrite,
    ExternalDeviceMutation,
    ManualPrecondition,
    RollbackRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowPhase {
    Planned,
    AwaitingConfirmation,
    Ready,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
