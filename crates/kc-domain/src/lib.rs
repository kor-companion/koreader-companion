pub mod addressing;
pub mod capability;
pub mod device;
pub mod error;
pub mod host;
pub mod logging;
pub mod payload;
pub mod persistence;
pub mod safety;
pub mod workflow;

pub use addressing::{Address, TransportKind};
pub use capability::{Capability, CapabilityProfile};
pub use device::{DeviceDescriptor, DeviceKind, DeviceTarget, ReadinessReport, SupportLevel};
pub use error::DomainError;
pub use host::{
    HostAccess, HostDescriptor, HostEjectResult, HostKind, HostOperationReadiness,
    HostOperationTarget, HostSyncResult, MetadataWriteRequest, MountPoint, ResourceKind,
    ResourceMetadata, ValidatedAddress, ValidatedPath,
};
pub use logging::{LogAttribution, LogSeverity, OperationLogEntry, OperationTarget};
pub use payload::{PayloadKind, PayloadRequest, PayloadSelection, PayloadSource};
pub use persistence::{
    BackupEntryKind, BackupManifestEntryRecord, BackupManifestRecord, CachedReleaseChannel,
    CachedReleaseMetadata, DeviceRecordRepository, KnownDeviceRecord, ManifestRepository,
    OperationLogQuery, OperationLogRepository, PersistenceStore, ReleaseArtifactRecord,
    ReleaseMetadataCache, StoredOperationLog,
};
pub use safety::{
    BackupPolicy, BackupPolicyAdvisor, BackupReference, BackupRequirement, ContainedPath,
    ContainmentPolicy, ProtectedWrite, RollbackGuidance, RollbackPlanner, RollbackStep,
    SafetyViolation,
};
pub use workflow::{
    CancellationSignal, ConfirmationGate, ConfirmationId, ConfirmationKind, DomainEvent,
    ExecutionId, ExecutionMode, OperationId, PlanId, PlanItem, PlanItemId, PlanItemKind,
    VerificationItem, VerificationReport, VerificationStatus, Workflow, WorkflowDefinition,
    WorkflowExecution, WorkflowExecutionResult, WorkflowExecutor, WorkflowKind, WorkflowPhase,
    WorkflowPlan, WorkflowPlanner, WorkflowProgressSink, WorkflowProgressUpdate, WorkflowSnapshot,
    WorkflowVerifier,
};
