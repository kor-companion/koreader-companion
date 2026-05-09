use std::path::PathBuf;

use kc_domain::{
    Address, CapabilityProfile, DeviceDescriptor, DeviceTarget, DomainError, MountPoint,
    ReadinessReport,
};

use crate::addressing::{contained_mount_address, contained_mount_path, normalize_relative_root};

#[derive(Debug, Clone)]
pub struct StaticDeviceTarget {
    descriptor: DeviceDescriptor,
    capabilities: CapabilityProfile,
    readiness: ReadinessReport,
    install_root: PathBuf,
    backup_root: PathBuf,
}

impl StaticDeviceTarget {
    pub fn new(
        descriptor: DeviceDescriptor,
        capabilities: CapabilityProfile,
        readiness: ReadinessReport,
        install_root: impl Into<PathBuf>,
        backup_root: impl Into<PathBuf>,
    ) -> Result<Self, DomainError> {
        let install_root = normalize_relative_root(install_root.into())?;
        let backup_root = normalize_relative_root(backup_root.into())?;

        Ok(Self {
            descriptor,
            capabilities,
            readiness,
            install_root,
            backup_root,
        })
    }
}

impl DeviceTarget for StaticDeviceTarget {
    fn descriptor(&self) -> &DeviceDescriptor {
        &self.descriptor
    }

    fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    fn readiness(&self, _mount: &MountPoint) -> Result<ReadinessReport, DomainError> {
        Ok(self.readiness.clone())
    }

    fn install_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError> {
        contained_mount_path(mount, &self.install_root)
    }

    fn backup_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError> {
        contained_mount_path(mount, &self.backup_root)
    }

    fn install_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        contained_mount_address(mount, &self.install_root)
    }

    fn backup_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        contained_mount_address(mount, &self.backup_root)
    }
}

#[cfg(test)]
mod tests {
    use kc_domain::{CapabilityProfile, DeviceKind, SupportLevel};

    use super::*;

    #[test]
    fn static_device_targets_reject_absolute_or_traversing_roots() {
        let descriptor = DeviceDescriptor {
            id: "future-target".to_string(),
            kind: DeviceKind::PocketBook,
            display_name: "Future target seam".to_string(),
            support_level: SupportLevel::Unsupported,
        };

        let absolute = StaticDeviceTarget::new(
            descriptor.clone(),
            CapabilityProfile::default(),
            ReadinessReport::blocked(vec!["not implemented".to_string()]),
            "/tmp/install",
            "system",
        )
        .unwrap_err();
        assert!(matches!(absolute, DomainError::Validation(_)));

        let traversing = StaticDeviceTarget::new(
            descriptor,
            CapabilityProfile::default(),
            ReadinessReport::blocked(vec!["not implemented".to_string()]),
            "../escape",
            "system",
        )
        .unwrap_err();
        assert!(matches!(traversing, DomainError::Validation(_)));
    }
}
