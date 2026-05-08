use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use kc_domain::{
    Capability, CapabilityProfile, ContainmentPolicy, DeviceDescriptor, DeviceKind, DeviceTarget,
    DomainError, HostAccess, MountPoint, ReadinessReport, SupportLevel,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceTargetDescriptor {
    pub descriptor: DeviceDescriptor,
    pub capabilities: CapabilityProfile,
}

pub fn supported_device_targets() -> Vec<DeviceTargetDescriptor> {
    vec![KoboTarget::<StdDeviceProbe>::descriptor_only()]
}

pub trait DeviceRootProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool;
    fn is_file(&self, root: &Path, relative: &Path) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StdDeviceProbe;

impl DeviceRootProbe for StdDeviceProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool {
        fs::metadata(root.join(relative))
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    fn is_file(&self, root: &Path, relative: &Path) -> bool {
        fs::metadata(root.join(relative))
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryDeviceProbe {
    directories: BTreeSet<PathBuf>,
    files: BTreeSet<PathBuf>,
}

impl InMemoryDeviceProbe {
    pub fn new(
        directories: impl IntoIterator<Item = PathBuf>,
        files: impl IntoIterator<Item = PathBuf>,
    ) -> Self {
        Self {
            directories: directories.into_iter().collect(),
            files: files.into_iter().collect(),
        }
    }
}

impl DeviceRootProbe for InMemoryDeviceProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool {
        self.directories.contains(&root.join(relative))
    }

    fn is_file(&self, root: &Path, relative: &Path) -> bool {
        self.files.contains(&root.join(relative))
    }
}

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
        DeviceTargetDescriptor {
            descriptor: DeviceDescriptor {
                id: "kobo-usb-mass-storage".to_string(),
                kind: DeviceKind::Kobo,
                display_name: "Kobo USB mass storage target".to_string(),
                support_level: SupportLevel::Supported,
            },
            capabilities: CapabilityProfile::new([
                Capability::CanInstallKOReader,
                Capability::CanBackupKOReaderData,
                Capability::CanRestoreKOReaderData,
                Capability::CanPatchLauncherConfig,
                Capability::SupportsDirectFilesystemAccess,
            ]),
        }
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
}

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
}

fn contained_mount_path(mount: &MountPoint, relative: &Path) -> Result<PathBuf, DomainError> {
    Ok(ContainmentPolicy::new(&mount.root)?
        .contain(relative)?
        .full_path)
}

fn normalize_relative_root(path: PathBuf) -> Result<PathBuf, DomainError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(DomainError::Validation(format!(
            "device target root must be a non-empty relative path: {}",
            path.display()
        )));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(DomainError::Validation(format!(
                    "device target root must stay within the mount boundary: {}",
                    path.display()
                )));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(DomainError::Validation(
            "device target root must not resolve to an empty path".to_string(),
        ));
    }

    Ok(normalized)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAssessment {
    pub mount: MountPoint,
    pub descriptor: DeviceDescriptor,
    pub readiness: ReadinessReport,
    pub install_root: PathBuf,
    pub backup_root: PathBuf,
}

pub fn assess_host_mounts(
    host: &dyn HostAccess,
    targets: &[&dyn DeviceTarget],
) -> Result<Vec<DeviceAssessment>, DomainError> {
    let mut assessments = Vec::new();

    for mount in host.discover_mounts()? {
        for target in targets {
            assessments.push(DeviceAssessment {
                mount: mount.clone(),
                descriptor: target.descriptor().clone(),
                readiness: target.readiness(&mount)?,
                install_root: target.install_root(&mount)?,
                backup_root: target.backup_root(&mount)?,
            });
        }
    }

    Ok(assessments)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

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
