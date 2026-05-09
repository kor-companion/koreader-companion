use std::collections::BTreeSet;

use crate::DomainError;

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
