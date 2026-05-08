use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use kc_domain::{
    Capability, CapabilityProfile, DomainError, HostAccess, HostDescriptor, HostKind, MountPoint,
    ValidatedPath,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostAdapterDescriptor {
    pub descriptor: HostDescriptor,
    pub capabilities: CapabilityProfile,
}

impl HostAdapterDescriptor {
    pub fn linux() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "linux-desktop".to_string(),
                kind: HostKind::Linux,
                display_name: "Linux desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([
                Capability::SupportsDirectFilesystemAccess,
                Capability::SupportsSafeEject,
            ]),
        }
    }

    pub fn macos() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "macos-desktop".to_string(),
                kind: HostKind::MacOs,
                display_name: "macOS desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }

    pub fn windows() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "windows-desktop".to_string(),
                kind: HostKind::Windows,
                display_name: "Windows desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }

    pub fn other(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            descriptor: HostDescriptor {
                kind: HostKind::Other(id.clone()),
                id,
                display_name: display_name.into(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }
}

pub fn supported_host_adapters() -> Vec<HostAdapterDescriptor> {
    vec![
        HostAdapterDescriptor::linux(),
        HostAdapterDescriptor::macos(),
        HostAdapterDescriptor::windows(),
    ]
}

pub fn current_host_adapter() -> HostAdapterDescriptor {
    match std::env::consts::OS {
        "linux" => HostAdapterDescriptor::linux(),
        "macos" => HostAdapterDescriptor::macos(),
        "windows" => HostAdapterDescriptor::windows(),
        other => HostAdapterDescriptor::other(other, format!("{other} host")),
    }
}

pub trait HostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StdHostFilesystem;

impl HostFilesystem for StdHostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError> {
        let canonical = fs::canonicalize(path).map_err(|error| {
            DomainError::Validation(format!(
                "failed to access manual path {}: {error}",
                path.display()
            ))
        })?;

        if !canonical.is_dir() {
            return Err(DomainError::Validation(format!(
                "manual path {} is not a directory",
                canonical.display()
            )));
        }

        Ok(canonical)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryHostFilesystem {
    directories: BTreeSet<PathBuf>,
}

impl InMemoryHostFilesystem {
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            directories: paths.into_iter().collect(),
        }
    }
}

impl HostFilesystem for InMemoryHostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError> {
        let candidate = path.to_path_buf();
        if self.directories.contains(&candidate) {
            Ok(candidate)
        } else {
            Err(DomainError::Validation(format!(
                "manual path {} is not available in the host fixture",
                path.display()
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilesystemHost<F = StdHostFilesystem> {
    descriptor: HostDescriptor,
    capabilities: CapabilityProfile,
    mounts: Vec<MountPoint>,
    filesystem: F,
}

impl<F> FilesystemHost<F> {
    pub fn with_mounts(
        adapter: HostAdapterDescriptor,
        filesystem: F,
        mounts: Vec<MountPoint>,
    ) -> Self {
        Self {
            descriptor: adapter.descriptor,
            capabilities: adapter.capabilities,
            mounts,
            filesystem,
        }
    }
}

impl<F: HostFilesystem> HostAccess for FilesystemHost<F> {
    fn descriptor(&self) -> &HostDescriptor {
        &self.descriptor
    }

    fn capabilities(&self) -> &CapabilityProfile {
        &self.capabilities
    }

    fn discover_mounts(&self) -> Result<Vec<MountPoint>, DomainError> {
        Ok(self.mounts.clone())
    }

    fn validate_manual_path(&self, path: &Path) -> Result<ValidatedPath, DomainError> {
        Ok(ValidatedPath {
            path: self.filesystem.canonicalize_dir(path)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_host_descriptors_are_exposed() {
        let adapters = supported_host_adapters();
        assert_eq!(adapters.len(), 3);
        assert_eq!(adapters[0].descriptor.kind, HostKind::Linux);
        assert_eq!(adapters[1].descriptor.kind, HostKind::MacOs);
        assert_eq!(adapters[2].descriptor.kind, HostKind::Windows);
    }

    #[test]
    fn filesystem_host_validates_manual_paths_through_boundary() {
        let host = FilesystemHost::with_mounts(
            HostAdapterDescriptor::linux(),
            InMemoryHostFilesystem::new([PathBuf::from("/mnt/kobo")]),
            vec![],
        );

        let validated = host.validate_manual_path(Path::new("/mnt/kobo")).unwrap();
        assert_eq!(validated.path, PathBuf::from("/mnt/kobo"));

        let error = host
            .validate_manual_path(Path::new("/mnt/missing"))
            .unwrap_err();
        assert!(matches!(error, DomainError::Validation(_)));
    }
}
