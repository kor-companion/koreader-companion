use crate::{DomainError, DomainEvent, LogAttribution};

use super::{
    DeviceRecordRepository, ManifestRepository, OperationLogRepository, ReleaseMetadataCache,
};

pub trait PersistenceStore:
    OperationLogRepository + DeviceRecordRepository + ManifestRepository + ReleaseMetadataCache
{
    fn record_event(
        &mut self,
        attribution: &LogAttribution,
        event: &DomainEvent,
    ) -> Result<(), DomainError>;
}
