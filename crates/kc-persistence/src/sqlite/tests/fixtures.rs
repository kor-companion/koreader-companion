use kc_domain::{
    Address, BackupEntryKind, BackupManifestEntryRecord, BackupManifestRecord,
    CachedReleaseChannel, CachedReleaseMetadata, ExecutionId, LogAttribution, LogSeverity,
    OperationId, OperationLogEntry, OperationTarget, PlanId, PlanItemId, ReleaseArtifactRecord,
    StoredOperationLog, TransportKind,
};

use crate::sqlite::schema::SCHEMA_VERSION;

pub(super) fn sample_log(severity: LogSeverity) -> StoredOperationLog {
    StoredOperationLog {
        entry: OperationLogEntry::new(
            LogAttribution {
                plan_id: PlanId::new(10),
                plan_item_id: PlanItemId::new(2),
                execution_id: ExecutionId::new(4),
                operation_id: OperationId::new(8),
                target: OperationTarget::from_address(
                    Address::scoped(TransportKind::UsbMassStorage, "kobo-main", ".adds/koreader")
                        .unwrap(),
                ),
            },
            severity,
            "staged payload",
        ),
        recorded_at_unix: 1_713_000_123,
    }
}

pub(super) fn sample_manifest() -> BackupManifestRecord {
    BackupManifestRecord {
        manifest_id: "backup-1".to_string(),
        device_id: "device-1".to_string(),
        created_at_unix: 1_713_000_300,
        profile: "default".to_string(),
        app_version: "0.1.0".to_string(),
        schema_version: SCHEMA_VERSION,
        source_root: Address::filesystem("/mnt/device"),
        entries: vec![BackupManifestEntryRecord {
            source: Address::filesystem("/mnt/device/.adds/koreader/settings.reader.lua"),
            backup: Address::scoped(
                TransportKind::LocalFilesystem,
                "backup-set-1",
                "payload/settings.reader.lua",
            )
            .unwrap(),
            kind: BackupEntryKind::File,
            size_bytes: 4096,
            checksum_hex: Some(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            ),
        }],
    }
}

pub(super) fn sample_release() -> CachedReleaseMetadata {
    CachedReleaseMetadata {
        cache_key: "stable-kobo".to_string(),
        release_id: "42".to_string(),
        version: "v2026.04".to_string(),
        channel: CachedReleaseChannel::Other("nightly-kobo".to_string()),
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
    }
}
