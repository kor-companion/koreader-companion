use crate::{
    Address, DeviceDescriptor, DomainError, DomainEvent, ExecutionId, LogAttribution, LogSeverity,
    OperationLogEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredOperationLog {
    pub entry: OperationLogEntry,
    pub recorded_at_unix: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OperationLogQuery {
    pub execution_id: Option<ExecutionId>,
    pub minimum_severity: Option<LogSeverity>,
}

impl OperationLogQuery {
    pub fn all() -> Self {
        Self::default()
    }
}

pub trait OperationLogRepository {
    fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), DomainError>;
    fn list_logs(&self, query: &OperationLogQuery) -> Result<Vec<StoredOperationLog>, DomainError>;
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupEntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifestEntryRecord {
    pub source: Address,
    pub backup: Address,
    pub kind: BackupEntryKind,
    pub size_bytes: u64,
    pub checksum_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifestRecord {
    pub manifest_id: String,
    pub device_id: String,
    pub created_at_unix: i64,
    pub profile: String,
    pub app_version: String,
    pub schema_version: i64,
    pub source_root: Address,
    pub entries: Vec<BackupManifestEntryRecord>,
}

pub trait ManifestRepository {
    fn save_manifest(&mut self, manifest: &BackupManifestRecord) -> Result<(), DomainError>;
    fn load_manifest(&self, id: &str) -> Result<Option<BackupManifestRecord>, DomainError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CachedReleaseChannel {
    Stable,
    Prerelease,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactRecord {
    pub name: String,
    pub download_url: String,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub checksum_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedReleaseMetadata {
    pub cache_key: String,
    pub release_id: String,
    pub version: String,
    pub channel: CachedReleaseChannel,
    pub published_at_unix: i64,
    pub fetched_at_unix: i64,
    pub source_url: String,
    pub artifacts: Vec<ReleaseArtifactRecord>,
}

pub trait ReleaseMetadataCache {
    fn put_release_metadata(&mut self, release: &CachedReleaseMetadata) -> Result<(), DomainError>;
    fn get_release_metadata(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, DomainError>;
}

pub trait PersistenceStore:
    OperationLogRepository + DeviceRecordRepository + ManifestRepository + ReleaseMetadataCache
{
    fn record_event(
        &mut self,
        attribution: &LogAttribution,
        event: &DomainEvent,
    ) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        Address, DeviceKind, ExecutionId, OperationId, OperationTarget, PlanId, PlanItemId,
        SupportLevel,
    };

    #[derive(Default)]
    struct InMemoryStore {
        logs: Vec<StoredOperationLog>,
        devices: BTreeMap<String, KnownDeviceRecord>,
        manifests: BTreeMap<String, BackupManifestRecord>,
        releases: BTreeMap<String, CachedReleaseMetadata>,
    }

    impl OperationLogRepository for InMemoryStore {
        fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), DomainError> {
            self.logs.push(entry.clone());
            Ok(())
        }

        fn list_logs(
            &self,
            query: &OperationLogQuery,
        ) -> Result<Vec<StoredOperationLog>, DomainError> {
            Ok(self
                .logs
                .iter()
                .filter(|record| {
                    query
                        .execution_id
                        .map(|execution_id| record.entry.attribution.execution_id == execution_id)
                        .unwrap_or(true)
                })
                .cloned()
                .collect())
        }
    }

    impl DeviceRecordRepository for InMemoryStore {
        fn upsert_device_record(&mut self, record: &KnownDeviceRecord) -> Result<(), DomainError> {
            self.devices
                .insert(record.descriptor.id.clone(), record.clone());
            Ok(())
        }

        fn get_device_record(&self, id: &str) -> Result<Option<KnownDeviceRecord>, DomainError> {
            Ok(self.devices.get(id).cloned())
        }
    }

    impl ManifestRepository for InMemoryStore {
        fn save_manifest(&mut self, manifest: &BackupManifestRecord) -> Result<(), DomainError> {
            self.manifests
                .insert(manifest.manifest_id.clone(), manifest.clone());
            Ok(())
        }

        fn load_manifest(&self, id: &str) -> Result<Option<BackupManifestRecord>, DomainError> {
            Ok(self.manifests.get(id).cloned())
        }
    }

    impl ReleaseMetadataCache for InMemoryStore {
        fn put_release_metadata(
            &mut self,
            release: &CachedReleaseMetadata,
        ) -> Result<(), DomainError> {
            self.releases
                .insert(release.cache_key.clone(), release.clone());
            Ok(())
        }

        fn get_release_metadata(
            &self,
            cache_key: &str,
        ) -> Result<Option<CachedReleaseMetadata>, DomainError> {
            Ok(self.releases.get(cache_key).cloned())
        }
    }

    #[test]
    fn repository_contracts_cover_logs_devices_manifests_and_release_cache() {
        let mut store = InMemoryStore::default();
        let log = StoredOperationLog {
            entry: OperationLogEntry::new(
                LogAttribution {
                    plan_id: PlanId::new(1),
                    plan_item_id: PlanItemId::new(2),
                    execution_id: ExecutionId::new(3),
                    operation_id: OperationId::new(4),
                    target: OperationTarget::from_address(Address::filesystem("/mnt/kobo")),
                },
                LogSeverity::Info,
                "planned",
            ),
            recorded_at_unix: 1_713_000_000,
        };
        store.append_log(&log).unwrap();
        assert_eq!(
            store.list_logs(&OperationLogQuery::all()).unwrap(),
            vec![log]
        );

        let device = KnownDeviceRecord {
            descriptor: DeviceDescriptor {
                id: "kobo-usb-mass-storage".to_string(),
                kind: DeviceKind::Kobo,
                display_name: "Kobo".to_string(),
                support_level: SupportLevel::Supported,
            },
            last_seen_at_unix: 1_713_000_100,
            last_host_id: Some("linux-desktop".to_string()),
            last_address: Some(Address::filesystem("/mnt/kobo")),
        };
        store.upsert_device_record(&device).unwrap();
        assert_eq!(
            store.get_device_record("kobo-usb-mass-storage").unwrap(),
            Some(device)
        );

        let manifest = BackupManifestRecord {
            manifest_id: "backup-1".to_string(),
            device_id: "kobo-usb-mass-storage".to_string(),
            created_at_unix: 1_713_000_200,
            profile: "full".to_string(),
            app_version: "0.1.0".to_string(),
            schema_version: 1,
            source_root: Address::filesystem("/mnt/kobo/.kobo"),
            entries: vec![BackupManifestEntryRecord {
                source: Address::filesystem("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf"),
                backup: Address::filesystem("/tmp/backup/Kobo eReader.conf"),
                kind: BackupEntryKind::File,
                size_bytes: 512,
                checksum_hex: Some("deadbeef".to_string()),
            }],
        };
        store.save_manifest(&manifest).unwrap();
        assert_eq!(store.load_manifest("backup-1").unwrap(), Some(manifest));

        let release = CachedReleaseMetadata {
            cache_key: "stable-kobo".to_string(),
            release_id: "42".to_string(),
            version: "v2026.04".to_string(),
            channel: CachedReleaseChannel::Stable,
            published_at_unix: 1_713_000_000,
            fetched_at_unix: 1_713_000_060,
            source_url: "https://example.invalid/releases/42".to_string(),
            artifacts: vec![ReleaseArtifactRecord {
                name: "koreader-kobo.zip".to_string(),
                download_url: "https://example.invalid/koreader-kobo.zip".to_string(),
                size_bytes: 10,
                content_type: Some("application/zip".to_string()),
                checksum_hex: None,
            }],
        };
        store.put_release_metadata(&release).unwrap();
        assert_eq!(
            store.get_release_metadata("stable-kobo").unwrap(),
            Some(release)
        );
    }
}
