use std::collections::BTreeMap;

use super::*;
use crate::{
    Address, DeviceKind, ExecutionId, LogAttribution, LogSeverity, OperationId, OperationLogEntry,
    OperationTarget, PlanId, PlanItemId, SupportLevel,
};

#[derive(Default)]
struct InMemoryStore {
    logs: Vec<StoredOperationLog>,
    devices: BTreeMap<String, KnownDeviceRecord>,
    manifests: BTreeMap<String, BackupManifestRecord>,
    releases: BTreeMap<String, CachedReleaseMetadata>,
}

impl OperationLogRepository for InMemoryStore {
    fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), crate::DomainError> {
        self.logs.push(entry.clone());
        Ok(())
    }

    fn list_logs(
        &self,
        query: &OperationLogQuery,
    ) -> Result<Vec<StoredOperationLog>, crate::DomainError> {
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
    fn upsert_device_record(
        &mut self,
        record: &KnownDeviceRecord,
    ) -> Result<(), crate::DomainError> {
        self.devices
            .insert(record.descriptor.id.clone(), record.clone());
        Ok(())
    }

    fn get_device_record(&self, id: &str) -> Result<Option<KnownDeviceRecord>, crate::DomainError> {
        Ok(self.devices.get(id).cloned())
    }
}

impl ManifestRepository for InMemoryStore {
    fn save_manifest(&mut self, manifest: &BackupManifestRecord) -> Result<(), crate::DomainError> {
        self.manifests
            .insert(manifest.manifest_id.clone(), manifest.clone());
        Ok(())
    }

    fn load_manifest(&self, id: &str) -> Result<Option<BackupManifestRecord>, crate::DomainError> {
        Ok(self.manifests.get(id).cloned())
    }
}

impl ReleaseMetadataCache for InMemoryStore {
    fn put_release_metadata(
        &mut self,
        release: &CachedReleaseMetadata,
    ) -> Result<(), crate::DomainError> {
        self.releases
            .insert(release.cache_key.clone(), release.clone());
        Ok(())
    }

    fn get_release_metadata(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, crate::DomainError> {
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
        descriptor: crate::DeviceDescriptor {
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
