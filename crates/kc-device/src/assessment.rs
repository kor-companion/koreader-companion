use std::path::PathBuf;

use kc_domain::{
    Address, DeviceDescriptor, DeviceTarget, DomainError, HostAccess, MountPoint, ReadinessReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAssessment {
    pub mount: MountPoint,
    pub descriptor: DeviceDescriptor,
    pub readiness: ReadinessReport,
    pub install_target: Address,
    pub backup_target: Address,
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
                install_target: target.install_target(&mount)?,
                backup_target: target.backup_target(&mount)?,
                install_root: target.install_root(&mount)?,
                backup_root: target.backup_root(&mount)?,
            });
        }
    }

    Ok(assessments)
}
