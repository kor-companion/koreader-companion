use crate::{Address, DeviceDescriptor, DomainError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownDeviceRecord {
    pub descriptor: DeviceDescriptor,
    pub last_seen_at_unix: i64,
    pub last_host_id: Option<String>,
    pub last_address: Option<Address>,
}

pub trait DeviceRecordRepository {
    fn upsert_device_record(&mut self, record: &KnownDeviceRecord) -> Result<(), DomainError>;
    fn get_device_record(&self, id: &str) -> Result<Option<KnownDeviceRecord>, DomainError>;
}
