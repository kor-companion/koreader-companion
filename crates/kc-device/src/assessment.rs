use std::path::PathBuf;

use kc_domain::{
    Address, DeviceDescriptor, DeviceTarget, DomainError, HostAccess, MountPoint, ReadinessReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAssessment {
    pub mount: MountPoint,
    pub descriptor: DeviceDescriptor,
    pub readiness: ReadinessReport,
    pub install_target: Option<Address>,
    pub backup_target: Option<Address>,
    pub install_root: Option<PathBuf>,
    pub backup_root: Option<PathBuf>,
}

pub fn assess_host_mounts(
    host: &dyn HostAccess,
    targets: &[&dyn DeviceTarget],
) -> Result<Vec<DeviceAssessment>, DomainError> {
    let mut assessments = Vec::new();

    for mount in host.discover_mounts()? {
        for target in targets {
            let readiness = target.readiness(&mount)?;
            let (install_target, backup_target, install_root, backup_root) = if readiness.ready {
                (
                    Some(target.install_target(&mount)?),
                    Some(target.backup_target(&mount)?),
                    Some(target.install_root(&mount)?),
                    Some(target.backup_root(&mount)?),
                )
            } else {
                (None, None, None, None)
            };

            assessments.push(DeviceAssessment {
                mount: mount.clone(),
                descriptor: target.descriptor().clone(),
                readiness,
                install_target,
                backup_target,
                install_root,
                backup_root,
            });
        }
    }

    Ok(assessments)
}
