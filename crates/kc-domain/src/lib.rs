use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Capability {
    CanInstallKOReader,
    CanBackupKOReaderData,
    CanRestoreKOReaderData,
    CanPatchLauncherConfig,
    RequiresJailbreak,
    RequiresDeveloperMode,
    SupportsSafeEject,
    SupportsDirectFilesystemAccess,
    SupportsRemoteShell,
    SupportsAdbInstall,
    SupportsSelectiveRestore,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilityProfile {
    supported: BTreeSet<Capability>,
}

impl CapabilityProfile {
    pub fn new<I>(capabilities: I) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        Self {
            supported: capabilities.into_iter().collect(),
        }
    }

    pub fn supports(&self, capability: Capability) -> bool {
        self.supported.contains(&capability)
    }

    pub fn missing(&self, required: &[Capability]) -> Vec<Capability> {
        required
            .iter()
            .copied()
            .filter(|capability| !self.supports(*capability))
            .collect()
    }

    pub fn ensure(
        &self,
        subject: impl Into<String>,
        required: &[Capability],
    ) -> Result<(), DomainError> {
        let missing = self.missing(required);
        if missing.is_empty() {
            Ok(())
        } else {
            Err(DomainError::MissingCapabilities {
                subject: subject.into(),
                missing,
            })
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.supported.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostKind {
    Linux,
    MacOs,
    Windows,
    Android,
    Ios,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostDescriptor {
    pub id: String,
    pub kind: HostKind,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountPoint {
    pub id: String,
    pub root: PathBuf,
    pub name: Option<String>,
    pub removable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPath {
    pub path: PathBuf,
}

pub trait HostAccess {
    fn descriptor(&self) -> &HostDescriptor;
    fn capabilities(&self) -> &CapabilityProfile;
    fn discover_mounts(&self) -> Result<Vec<MountPoint>, DomainError>;
    fn validate_manual_path(&self, path: &Path) -> Result<ValidatedPath, DomainError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceKind {
    Kobo,
    PocketBook,
    Kindle,
    Android,
    Remarkable,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportLevel {
    Supported,
    Experimental,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub id: String,
    pub kind: DeviceKind,
    pub display_name: String,
    pub support_level: SupportLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReadinessReport {
    pub ready: bool,
    pub blockers: Vec<String>,
}

impl ReadinessReport {
    pub fn ready() -> Self {
        Self {
            ready: true,
            blockers: Vec::new(),
        }
    }

    pub fn blocked(blockers: Vec<String>) -> Self {
        Self {
            ready: false,
            blockers,
        }
    }
}

pub trait DeviceTarget {
    fn descriptor(&self) -> &DeviceDescriptor;
    fn capabilities(&self) -> &CapabilityProfile;
    fn readiness(&self, mount: &MountPoint) -> Result<ReadinessReport, DomainError>;
    fn install_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError>;
    fn backup_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    ReleaseArtifact,
    BackupArchive,
    LocalDirectory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadRequest {
    pub workflow: WorkflowKind,
    pub identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadSelection {
    pub kind: PayloadKind,
    pub identifier: String,
    pub display_name: String,
}

pub trait PayloadSource {
    fn kind(&self) -> PayloadKind;
    fn resolve(&self, request: &PayloadRequest) -> Result<PayloadSelection, DomainError>;
}

pub trait PersistenceStore {
    fn record_event(
        &mut self,
        attribution: &LogAttribution,
        event: &DomainEvent,
    ) -> Result<(), DomainError>;
}

pub trait Workflow {
    fn kind(&self) -> WorkflowKind;
    fn plan(&self) -> Result<WorkflowPlan, DomainError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanId(u64);

impl PlanId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanItemId(u64);

impl PlanItemId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionId(u64);

impl ExecutionId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId(u64);

impl OperationId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfirmationId(u64);

impl ConfirmationId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationTarget {
    HostPath(PathBuf),
    DevicePath(PathBuf),
    Payload(String),
    Logical(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationKind {
    DestructiveWrite,
    ExternalDeviceMutation,
    ManualPrecondition,
    RollbackRisk,
}

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
pub enum DomainEvent {
    WorkflowPlanned(WorkflowSnapshot),
    ConfirmationRequired(ConfirmationGate, WorkflowSnapshot),
    PhaseChanged(WorkflowPhase, WorkflowSnapshot),
    PlanItemStarted(PlanItemId, WorkflowSnapshot),
    PlanItemCompleted(PlanItemId, WorkflowSnapshot),
    WorkflowFinished(WorkflowSnapshot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainedPath {
    pub root: PathBuf,
    pub full_path: PathBuf,
    pub relative_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainmentPolicy {
    root: PathBuf,
}

impl ContainmentPolicy {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, SafetyViolation> {
        let root = root.into();
        if !root.is_absolute() {
            return Err(SafetyViolation::RootMustBeAbsolute(root));
        }

        Ok(Self {
            root: normalize_root(&root)?,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn contain(&self, candidate: &Path) -> Result<ContainedPath, SafetyViolation> {
        match fs::symlink_metadata(&self.root) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(SafetyViolation::SymlinkComponent(self.root.clone()));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(SafetyViolation::PathResolution {
                    path: self.root.clone(),
                    message: error.to_string(),
                });
            }
        }

        let full_path = if candidate.is_absolute() {
            normalize_path(candidate)?
        } else {
            normalize_path(&self.root.join(candidate))?
        };

        if !full_path.starts_with(&self.root) {
            return Err(SafetyViolation::PathOutsideRoot {
                root: self.root.clone(),
                candidate: full_path,
            });
        }

        reject_symlink_components(&self.root, &full_path)?;

        let relative_path = full_path
            .strip_prefix(&self.root)
            .unwrap_or_else(|_| Path::new(""))
            .to_path_buf();

        Ok(ContainedPath {
            root: self.root.clone(),
            full_path,
            relative_path,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyViolation {
    RootMustBeAbsolute(PathBuf),
    PathTraversal(PathBuf),
    PathOutsideRoot { root: PathBuf, candidate: PathBuf },
    SymlinkComponent(PathBuf),
    PathResolution { path: PathBuf, message: String },
}

impl fmt::Display for SafetyViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafetyViolation::RootMustBeAbsolute(path) => {
                write!(f, "containment root must be absolute: {}", path.display())
            }
            SafetyViolation::PathTraversal(path) => {
                write!(f, "path escapes containment root: {}", path.display())
            }
            SafetyViolation::PathOutsideRoot { root, candidate } => write!(
                f,
                "path {} is outside containment root {}",
                candidate.display(),
                root.display()
            ),
            SafetyViolation::SymlinkComponent(path) => {
                write!(
                    f,
                    "path component resolves through a symlink: {}",
                    path.display()
                )
            }
            SafetyViolation::PathResolution { path, message } => {
                write!(f, "failed to resolve path {}: {message}", path.display())
            }
        }
    }
}

impl Error for SafetyViolation {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogAttribution {
    pub plan_id: PlanId,
    pub plan_item_id: PlanItemId,
    pub execution_id: ExecutionId,
    pub operation_id: OperationId,
    pub target: OperationTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationLogEntry {
    pub attribution: LogAttribution,
    pub severity: LogSeverity,
    pub message: String,
}

impl OperationLogEntry {
    pub fn new(
        attribution: LogAttribution,
        severity: LogSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            attribution,
            severity,
            message: message.into(),
        }
    }
}

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
            DomainError::MissingCapabilities { subject, missing } => {
                write!(f, "{subject} is missing capabilities: {missing:?}")
            }
            DomainError::InvalidWorkflowTransition { from, to } => {
                write!(f, "invalid workflow transition from {from:?} to {to:?}")
            }
            DomainError::DuplicatePlanItem(id) => {
                write!(f, "duplicate plan item id {}", id.0)
            }
            DomainError::DuplicateConfirmation(id) => {
                write!(f, "duplicate confirmation id {}", id.0)
            }
            DomainError::UnknownConfirmation(id) => {
                write!(f, "unknown confirmation id {}", id.0)
            }
            DomainError::UnknownPlanItem(id) => write!(f, "unknown plan item id {}", id.0),
            DomainError::Safety(error) => write!(f, "{error}"),
            DomainError::Validation(message) | DomainError::Unsupported(message) => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for DomainError {}

impl From<SafetyViolation> for DomainError {
    fn from(value: SafetyViolation) -> Self {
        Self::Safety(value)
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

fn normalize_root(path: &Path) -> Result<PathBuf, SafetyViolation> {
    if path.exists() {
        fs::canonicalize(path).map_err(|error| SafetyViolation::PathResolution {
            path: path.to_path_buf(),
            message: error.to_string(),
        })
    } else {
        normalize_path(path)
    }
}

fn reject_symlink_components(root: &Path, full_path: &Path) -> Result<(), SafetyViolation> {
    let relative = full_path
        .strip_prefix(root)
        .map_err(|_| SafetyViolation::PathOutsideRoot {
            root: root.to_path_buf(),
            candidate: full_path.to_path_buf(),
        })?;

    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());

        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(SafetyViolation::SymlinkComponent(current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => {
                return Err(SafetyViolation::PathResolution {
                    path: current,
                    message: error.to_string(),
                });
            }
        }
    }

    Ok(())
}

fn normalize_path(path: &Path) -> Result<PathBuf, SafetyViolation> {
    let mut normalized = PathBuf::new();
    let mut floor = 0usize;

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => {
                normalized.push(prefix.as_os_str());
                floor = normalized.components().count();
            }
            Component::RootDir => {
                normalized.push(component.as_os_str());
                floor = normalized.components().count();
            }
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                if normalized.components().count() <= floor {
                    return Err(SafetyViolation::PathTraversal(path.to_path_buf()));
                }
                normalized.pop();
            }
        }
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    struct DummyHost {
        descriptor: HostDescriptor,
        capabilities: CapabilityProfile,
    }

    impl HostAccess for DummyHost {
        fn descriptor(&self) -> &HostDescriptor {
            &self.descriptor
        }

        fn capabilities(&self) -> &CapabilityProfile {
            &self.capabilities
        }

        fn discover_mounts(&self) -> Result<Vec<MountPoint>, DomainError> {
            Ok(vec![MountPoint {
                id: "primary".to_string(),
                root: PathBuf::from("/mnt/kobo"),
                name: Some("Kobo".to_string()),
                removable: true,
            }])
        }

        fn validate_manual_path(&self, path: &Path) -> Result<ValidatedPath, DomainError> {
            Ok(ValidatedPath {
                path: path.to_path_buf(),
            })
        }
    }

    #[test]
    fn capability_contracts_report_missing_requirements() {
        let host = DummyHost {
            descriptor: HostDescriptor {
                id: "linux-host".to_string(),
                kind: HostKind::Linux,
                display_name: "Linux".to_string(),
            },
            capabilities: CapabilityProfile::new([
                Capability::SupportsDirectFilesystemAccess,
                Capability::SupportsSafeEject,
            ]),
        };

        assert!(host
            .capabilities()
            .supports(Capability::SupportsDirectFilesystemAccess));

        let error = host
            .capabilities()
            .ensure(
                "install workflow",
                &[
                    Capability::CanInstallKOReader,
                    Capability::SupportsDirectFilesystemAccess,
                ],
            )
            .unwrap_err();

        assert_eq!(
            error,
            DomainError::MissingCapabilities {
                subject: "install workflow".to_string(),
                missing: vec![Capability::CanInstallKOReader],
            }
        );
        assert_eq!(host.discover_mounts().unwrap().len(), 1);
    }

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
    fn containment_policy_rejects_escape_paths() {
        let policy = ContainmentPolicy::new("/mnt/kobo").unwrap();

        let contained = policy
            .contain(Path::new(".adds/../.adds/koreader"))
            .unwrap();
        assert_eq!(
            contained.full_path,
            PathBuf::from("/mnt/kobo/.adds/koreader")
        );
        assert_eq!(contained.relative_path, PathBuf::from(".adds/koreader"));

        let error = policy.contain(Path::new("../etc/passwd")).unwrap_err();
        assert_eq!(
            error,
            SafetyViolation::PathOutsideRoot {
                root: PathBuf::from("/mnt/kobo"),
                candidate: PathBuf::from("/mnt/etc/passwd"),
            }
        );
    }

    #[cfg(unix)]
    #[test]
    fn containment_policy_rejects_symlink_components() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("kc-domain-{unique}"));
        let outside = std::env::temp_dir().join(format!("kc-domain-outside-{unique}"));

        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        symlink(&outside, root.join("escape")).unwrap();

        let policy = ContainmentPolicy::new(&root).unwrap();
        let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
        assert_eq!(
            error,
            SafetyViolation::SymlinkComponent(root.join("escape"))
        );

        fs::remove_file(root.join("escape")).unwrap();
        fs::remove_dir_all(&root).unwrap();
        fs::remove_dir_all(&outside).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn containment_policy_rejects_root_symlink_created_after_init() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("kc-domain-late-root-{unique}"));
        let outside = std::env::temp_dir().join(format!("kc-domain-late-outside-{unique}"));

        let policy = ContainmentPolicy::new(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        symlink(&outside, &root).unwrap();

        let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
        assert_eq!(error, SafetyViolation::SymlinkComponent(root.clone()));

        fs::remove_file(&root).unwrap();
        fs::remove_dir_all(&outside).unwrap();
    }

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

    #[test]
    fn log_attribution_stays_tied_to_plan_items() {
        let item = PlanItem::new(
            PlanItemId::new(11),
            PlanItemKind::Write,
            "Install launcher hook",
            OperationTarget::DevicePath(PathBuf::from("/.kobo/KoboRoot.tgz")),
        );

        let attribution =
            item.log_attribution(PlanId::new(3), ExecutionId::new(9), OperationId::new(15));
        let entry =
            OperationLogEntry::new(attribution.clone(), LogSeverity::Info, "write scheduled");

        assert_eq!(attribution.plan_id, PlanId::new(3));
        assert_eq!(attribution.plan_item_id, PlanItemId::new(11));
        assert_eq!(attribution.execution_id, ExecutionId::new(9));
        assert_eq!(
            entry.attribution.target,
            OperationTarget::DevicePath(PathBuf::from("/.kobo/KoboRoot.tgz"))
        );
    }
}
