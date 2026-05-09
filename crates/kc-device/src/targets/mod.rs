mod descriptor;
mod kobo;
mod static_target;

pub use descriptor::{supported_device_targets, DeviceTargetDescriptor};
pub use kobo::KoboTarget;
pub use static_target::StaticDeviceTarget;
