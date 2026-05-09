use std::path::Path;

use crate::{
    Address, CapabilityProfile, DomainError, HostDescriptor, HostEjectResult,
    HostOperationReadiness, HostOperationTarget, HostSyncResult, MetadataWriteRequest, MountPoint,
    ResourceMetadata, ValidatedAddress, ValidatedPath,
};

pub trait HostAccess {
    fn descriptor(&self) -> &HostDescriptor;
    fn capabilities(&self) -> &CapabilityProfile;
    fn discover_mounts(&self) -> Result<Vec<MountPoint>, DomainError>;
    fn validate_manual_path(&self, path: &Path) -> Result<ValidatedPath, DomainError>;

    fn validate_manual_address(&self, address: &Address) -> Result<ValidatedAddress, DomainError> {
        match address {
            Address::LocalPath(path) => Ok(ValidatedAddress {
                address: self.validate_manual_path(path)?.address(),
            }),
            _ => Err(DomainError::Unsupported(
                "manual host selection is not supported for non-filesystem addresses".to_string(),
            )),
        }
    }

    fn read_metadata(&self, _address: &Address) -> Result<ResourceMetadata, DomainError> {
        Err(DomainError::Unsupported(
            "host metadata reads are not implemented by this adapter".to_string(),
        ))
    }

    fn write_metadata(
        &self,
        _request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError> {
        Err(DomainError::Unsupported(
            "host metadata writes are not implemented by this adapter".to_string(),
        ))
    }

    fn sync_readiness(
        &self,
        _target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        Ok(HostOperationReadiness::unsupported(
            "host sync readiness is not implemented by this adapter",
        ))
    }

    fn sync(&self, _target: &HostOperationTarget) -> Result<HostSyncResult, DomainError> {
        Err(DomainError::Unsupported(
            "host sync is not implemented by this adapter".to_string(),
        ))
    }

    fn eject_readiness(
        &self,
        _target: &HostOperationTarget,
    ) -> Result<HostOperationReadiness, DomainError> {
        Ok(HostOperationReadiness::unsupported(
            "host eject readiness is not implemented by this adapter",
        ))
    }

    fn eject(&self, _target: &HostOperationTarget) -> Result<HostEjectResult, DomainError> {
        Err(DomainError::Unsupported(
            "host eject is not implemented by this adapter".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::HostAccess;
    use crate::{
        Address, Capability, CapabilityProfile, DomainError, HostDescriptor, HostKind, MountPoint,
        ValidatedPath,
    };

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
    fn manual_address_validation_delegates_local_paths() {
        let host = DummyHost {
            descriptor: HostDescriptor {
                id: "desktop".to_string(),
                kind: HostKind::Linux,
                display_name: "Desktop".to_string(),
            },
            capabilities: CapabilityProfile::default(),
        };

        let validated = host
            .validate_manual_address(&Address::filesystem("/mnt/kobo"))
            .unwrap();
        assert_eq!(validated.address, Address::filesystem("/mnt/kobo"));

        assert!(matches!(
            host.validate_manual_address(
                &Address::remote(crate::TransportKind::Ssh, "host", "/mnt/kobo").unwrap()
            ),
            Err(DomainError::Unsupported(_))
        ));
    }
}
