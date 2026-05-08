use crate::{Address, DomainError, OperationTarget, WorkflowSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupPolicy {
    Required,
    Recommended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupRequirement {
    pub policy: BackupPolicy,
    pub target: Address,
    pub reason: String,
}

impl BackupRequirement {
    pub fn required(target: Address, reason: impl Into<String>) -> Self {
        Self {
            policy: BackupPolicy::Required,
            target,
            reason: reason.into(),
        }
    }

    pub fn recommended(target: Address, reason: impl Into<String>) -> Self {
        Self {
            policy: BackupPolicy::Recommended,
            target,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedWrite {
    pub target: OperationTarget,
    pub destructive: bool,
    pub backup: Option<BackupRequirement>,
}

impl ProtectedWrite {
    pub fn new(target: OperationTarget) -> Self {
        Self {
            target,
            destructive: false,
            backup: None,
        }
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn with_backup(mut self, backup: BackupRequirement) -> Self {
        self.backup = Some(backup);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupReference {
    pub backup_id: String,
    pub manifest_id: Option<String>,
    pub location: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackStep {
    pub summary: String,
    pub target: Option<Address>,
    pub requires_manual_action: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackGuidance {
    pub summary: String,
    pub steps: Vec<RollbackStep>,
    pub backup: Option<BackupReference>,
}

pub trait BackupPolicyAdvisor {
    fn backup_requirement(
        &self,
        write: &ProtectedWrite,
    ) -> Result<Option<BackupRequirement>, DomainError>;
}

pub trait RollbackPlanner {
    fn rollback_guidance(
        &self,
        snapshot: &WorkflowSnapshot,
    ) -> Result<RollbackGuidance, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Address, ExecutionMode, OperationTarget, PlanId, WorkflowKind, WorkflowPhase};

    #[test]
    fn protected_writes_can_carry_backup_guidance() {
        let backup = BackupRequirement::required(
            Address::filesystem("/mnt/kobo/.kobo"),
            "Preserve launcher state before patching",
        );
        let write = ProtectedWrite::new(OperationTarget::from_address(Address::filesystem(
            "/mnt/kobo/.adds/koreader",
        )))
        .destructive(true)
        .with_backup(backup.clone());

        assert!(write.destructive);
        assert_eq!(write.backup, Some(backup));
    }

    #[test]
    fn rollback_guidance_keeps_manual_recovery_steps() {
        let snapshot = WorkflowSnapshot {
            plan_id: PlanId::new(5),
            kind: WorkflowKind::Install,
            mode: ExecutionMode::Guarded,
            phase: WorkflowPhase::Failed,
            total_items: 2,
            completed_items: 1,
            active_item: None,
            pending_confirmations: Vec::new(),
            failure_message: Some("sync failed".to_string()),
        };

        struct DummyRollback;

        impl RollbackPlanner for DummyRollback {
            fn rollback_guidance(
                &self,
                snapshot: &WorkflowSnapshot,
            ) -> Result<RollbackGuidance, DomainError> {
                Ok(RollbackGuidance {
                    summary: format!("Recover {:?} workflow", snapshot.kind),
                    steps: vec![RollbackStep {
                        summary: "Restore the preserved launcher backup".to_string(),
                        target: Some(Address::filesystem("/mnt/kobo/.kobo")),
                        requires_manual_action: true,
                    }],
                    backup: Some(BackupReference {
                        backup_id: "backup-1".to_string(),
                        manifest_id: Some("manifest-1".to_string()),
                        location: Some(Address::filesystem("/tmp/backup-1")),
                    }),
                })
            }
        }

        let guidance = DummyRollback.rollback_guidance(&snapshot).unwrap();
        assert_eq!(guidance.steps.len(), 1);
        assert!(guidance.steps[0].requires_manual_action);
    }
}
