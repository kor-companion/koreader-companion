use std::path::PathBuf;

use crate::{Address, CapabilityProfile, DomainError, MountPoint};

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

    fn install_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        Ok(Address::filesystem(self.install_root(mount)?))
    }

    fn backup_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        Ok(Address::filesystem(self.backup_root(mount)?))
    }
}
