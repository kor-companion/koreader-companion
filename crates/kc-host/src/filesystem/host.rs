use std::path::Path;

use kc_domain::{
    Address, Capability, CapabilityProfile, DomainError, HostAccess, HostDescriptor,
    HostEjectResult, HostOperationReadiness, HostOperationTarget, HostSyncResult,
    MetadataWriteRequest, MountPoint, ResourceMetadata, ValidatedPath,
};

use super::access::{HostFilesystem, StdHostFilesystem};
use crate::HostAdapterDescriptor;

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

impl<F: HostFilesystem> FilesystemHost<F> {
    fn sync_guidance(&self) -> Vec<String> {
        vec![format!(
            "use the host OS sync or safe-removal flow manually for {} until automated sync is implemented",
            self.descriptor.display_name
        )]
    }

    fn eject_guidance(&self) -> Vec<String> {
        vec![format!(
            "use the host OS eject UI or desktop tooling manually for {} until automated eject is implemented",
            self.descriptor.display_name
        )]
    }

    fn assess_target_exists(&self, target: &HostOperationTarget) -> Result<(), DomainError> {
        let address = match target {
            HostOperationTarget::Mount(mount) => mount.root_address(),
            HostOperationTarget::Address(address) => address.clone(),
        };

        let metadata = self.filesystem.read_metadata(&address)?;
        if metadata.exists {
            Ok(())
        } else {
            Err(DomainError::Validation(format!(
                "host operation target is not available: {}",
                format_target(target)
            )))
        }
    }

    fn sync_readiness_for(
        &self,
        target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        self.assess_target_exists(target)?;

        Ok(HostOperationReadiness::blocked(
            vec![format!(
                "automatic host sync is not implemented by {}",
                self.descriptor.id
            )],
            self.sync_guidance(),
        ))
    }

    fn eject_readiness_for(
        &self,
        target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        match target {
            HostOperationTarget::Mount(mount) if !mount.removable => {
                return Ok(HostOperationReadiness::blocked(
                    vec![format!("mount {} is not marked removable", mount.id)],
                    self.eject_guidance(),
                ));
            }
            HostOperationTarget::Address(_) => {
                return Ok(HostOperationReadiness::blocked(
                    vec![
                        "eject readiness currently requires a discovered or manually selected mount"
                            .to_string(),
                    ],
                    self.eject_guidance(),
                ));
            }
            HostOperationTarget::Mount(_) => {}
        }

        self.assess_target_exists(target)?;

        if !self.capabilities.supports(Capability::SupportsSafeEject) {
            return Ok(HostOperationReadiness::unsupported(format!(
                "{} does not yet expose an automated safe eject implementation",
                self.descriptor.display_name
            )));
        }

        Ok(HostOperationReadiness::blocked(
            vec![format!(
                "automatic host eject is not implemented by {}",
                self.descriptor.id
            )],
            self.eject_guidance(),
        ))
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

    fn read_metadata(&self, address: &Address) -> Result<ResourceMetadata, DomainError> {
        self.filesystem.read_metadata(address)
    }

    fn write_metadata(
        &self,
        request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError> {
        self.filesystem.write_metadata(request)
    }

    fn sync_readiness(
        &self,
        target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        self.sync_readiness_for(target)
    }

    fn sync(&self, target: &HostOperationTarget) -> Result<HostSyncResult, DomainError> {
        let _ = self.sync_readiness_for(target)?;
        Err(DomainError::Unsupported(format!(
            "{} reports manual sync guidance only; automated host sync is not implemented",
            self.descriptor.display_name
        )))
    }

    fn eject_readiness(
        &self,
        target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        self.eject_readiness_for(target)
    }

    fn eject(&self, target: &HostOperationTarget) -> Result<HostEjectResult, DomainError> {
        let _ = self.eject_readiness_for(target)?;
        Err(DomainError::Unsupported(format!(
            "{} reports manual eject guidance only; automated host eject is not implemented",
            self.descriptor.display_name
        )))
    }
}

fn format_target(target: &HostOperationTarget) -> String {
    match target {
        HostOperationTarget::Mount(mount) => mount.root.display().to_string(),
        HostOperationTarget::Address(address) => match address {
            Address::LocalPath(path) => path.display().to_string(),
            Address::ScopedPath {
                transport,
                scope,
                relative_path,
            } => format!("{transport:?}:{scope}:{}", relative_path.display()),
            Address::Remote {
                transport,
                locator,
                path,
            } => format!("{transport:?}:{locator}:{path}"),
            Address::Logical { scheme, value } => format!("{scheme}:{value}"),
        },
    }
}
