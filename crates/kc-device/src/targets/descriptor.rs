use kc_domain::{Capability, CapabilityProfile, DeviceDescriptor, DeviceKind, SupportLevel};

use super::kobo::KoboTarget;
use crate::StdDeviceProbe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceTargetDescriptor {
    pub descriptor: DeviceDescriptor,
    pub capabilities: CapabilityProfile,
}

pub fn supported_device_targets() -> Vec<DeviceTargetDescriptor> {
    vec![KoboTarget::<StdDeviceProbe>::descriptor_only()]
}

pub fn kobo_descriptor() -> DeviceTargetDescriptor {
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
