use std::path::{Path, PathBuf};

use kc_domain::{
    Address, CapabilityProfile, DeviceDescriptor, DeviceTarget, DomainError, MountPoint,
    ReadinessReport,
};

use super::descriptor::{kobo_descriptor, DeviceTargetDescriptor};
use crate::addressing::{contained_mount_address, contained_mount_path};
use crate::probe::{DeviceRootProbe, StdDeviceProbe};

#[derive(Debug, Clone)]
pub struct KoboTarget<P = StdDeviceProbe> {
    descriptor: DeviceDescriptor,
    capabilities: CapabilityProfile,
    probe: P,
}

impl<P> KoboTarget<P> {
    pub fn new(probe: P) -> Self {
        let descriptor = Self::descriptor_only();
        Self {
            descriptor: descriptor.descriptor,
            capabilities: descriptor.capabilities,
            probe,
        }
    }

    pub fn descriptor_only() -> DeviceTargetDescriptor {
        kobo_descriptor()
    }
}

impl<P: DeviceRootProbe> DeviceTarget for KoboTarget<P> {
    fn descriptor(&self) -> &DeviceDescriptor {
        &self.descriptor
    }

    fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    fn readiness(&self, mount: &MountPoint) -> Result<ReadinessReport, DomainError> {
        let mut blockers = Vec::new();

        if !self.probe.is_dir(&mount.root, Path::new(".kobo")) {
            blockers.push("missing required .kobo directory".to_string());
        }

        if !self
            .probe
            .is_file(&mount.root, Path::new(".kobo/Kobo/Kobo eReader.conf"))
        {
            blockers.push("missing Kobo config .kobo/Kobo/Kobo eReader.conf".to_string());
        }

        if blockers.is_empty() {
            Ok(ReadinessReport::ready())
        } else {
            Ok(ReadinessReport::blocked(blockers))
        }
    }

    fn install_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError> {
        contained_mount_path(mount, Path::new(".adds/koreader"))
    }

    fn backup_root(&self, mount: &MountPoint) -> Result<PathBuf, DomainError> {
        contained_mount_path(mount, Path::new(".kobo"))
    }

    fn install_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        contained_mount_address(mount, Path::new(".adds/koreader"))
    }

    fn backup_target(&self, mount: &MountPoint) -> Result<Address, DomainError> {
        contained_mount_address(mount, Path::new(".kobo"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryDeviceProbe;

    #[test]
    fn kobo_target_reports_readiness_from_probe_boundary() {
        let target = KoboTarget::new(InMemoryDeviceProbe::new(
            [PathBuf::from("/mnt/kobo/.kobo")],
            [PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf")],
        ));
        let mount = MountPoint {
            id: "kobo".to_string(),
            root: PathBuf::from("/mnt/kobo"),
            name: Some("KOBOeReader".to_string()),
            removable: true,
        };

        let readiness = target.readiness(&mount).unwrap();
        assert!(readiness.ready);
        assert!(readiness.blockers.is_empty());
        assert_eq!(
            target.install_root(&mount).unwrap(),
            PathBuf::from("/mnt/kobo/.adds/koreader")
        );
        assert!(matches!(
            target.install_target(&mount).unwrap(),
            Address::ScopedPath { .. }
        ));
    }

    #[test]
    fn kobo_target_does_not_treat_symlink_probe_entries_as_ready() {
        let target = KoboTarget::new(InMemoryDeviceProbe::new([], []).with_symlinks([
            PathBuf::from("/mnt/kobo/.kobo"),
            PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf"),
        ]));
        let mount = MountPoint {
            id: "kobo".to_string(),
            root: PathBuf::from("/mnt/kobo"),
            name: Some("KOBOeReader".to_string()),
            removable: true,
        };

        let readiness = target.readiness(&mount).unwrap();
        assert!(!readiness.ready);
        assert_eq!(readiness.blockers.len(), 2);
    }
}
