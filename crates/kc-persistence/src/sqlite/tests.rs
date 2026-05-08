use kc_domain::{
    Address, BackupEntryKind, BackupManifestEntryRecord, BackupManifestRecord,
    CachedReleaseChannel, CachedReleaseMetadata, ConfirmationGate, ConfirmationId,
    DeviceDescriptor, DeviceKind, DomainEvent, ExecutionId, ExecutionMode, KnownDeviceRecord,
    LogAttribution, LogSeverity, ManifestRepository, OperationId, OperationLogEntry,
    OperationLogQuery, OperationLogRepository, OperationTarget, PersistenceStore, PlanId,
    PlanItemId, ReleaseArtifactRecord, ReleaseMetadataCache, StoredOperationLog, SupportLevel,
    TransportKind, VerificationReport, WorkflowKind, WorkflowPhase,
};

use crate::sqlite::schema::SCHEMA_VERSION;
use crate::sqlite::store::SqliteStore;
use crate::PersistenceError;

fn sample_log(severity: LogSeverity) -> StoredOperationLog {
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

fn sample_manifest() -> BackupManifestRecord {
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

fn sample_release() -> CachedReleaseMetadata {
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

#[test]
fn bootstraps_schema_version() {
    let store = SqliteStore::in_memory().unwrap();
    assert_eq!(store.schema_version().unwrap(), SCHEMA_VERSION);
}

#[test]
fn repository_traits_cover_logs_devices_manifests_release_cache_and_events() {
    let mut store = SqliteStore::in_memory().unwrap();

    OperationLogRepository::append_log(&mut store, &sample_log(LogSeverity::Info)).unwrap();
    OperationLogRepository::append_log(&mut store, &sample_log(LogSeverity::Error)).unwrap();
    let filtered = OperationLogRepository::list_logs(
        &store,
        &OperationLogQuery {
            execution_id: Some(ExecutionId::new(4)),
            minimum_severity: Some(LogSeverity::Warning),
        },
    )
    .unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].entry.severity, LogSeverity::Error);

    let device = KnownDeviceRecord {
        descriptor: DeviceDescriptor {
            id: "kobo-usb".to_string(),
            kind: DeviceKind::Other("boox".to_string()),
            display_name: "Reader".to_string(),
            support_level: SupportLevel::Experimental,
        },
        last_seen_at_unix: 1_713_000_100,
        last_host_id: Some("linux-desktop".to_string()),
        last_address: Some(
            Address::scoped(TransportKind::UsbMassStorage, "kobo-usb", ".kobo").unwrap(),
        ),
    };
    kc_domain::DeviceRecordRepository::upsert_device_record(&mut store, &device).unwrap();
    assert_eq!(
        kc_domain::DeviceRecordRepository::get_device_record(&store, "kobo-usb").unwrap(),
        Some(device)
    );

    let manifest = sample_manifest();
    ManifestRepository::save_manifest(&mut store, &manifest).unwrap();
    assert_eq!(
        ManifestRepository::load_manifest(&store, "backup-1").unwrap(),
        Some(manifest)
    );

    let release = sample_release();
    ReleaseMetadataCache::put_release_metadata(&mut store, &release).unwrap();
    assert_eq!(
        ReleaseMetadataCache::get_release_metadata(&store, "stable-kobo").unwrap(),
        Some(release)
    );

    let snapshot = kc_domain::WorkflowSnapshot {
        plan_id: PlanId::new(10),
        kind: WorkflowKind::Install,
        mode: ExecutionMode::Guarded,
        phase: WorkflowPhase::AwaitingConfirmation,
        total_items: 1,
        completed_items: 0,
        active_item: Some(PlanItemId::new(2)),
        pending_confirmations: vec![ConfirmationGate::for_plan_item(
            ConfirmationId::new(1),
            kc_domain::ConfirmationKind::ExternalDeviceMutation,
            "Confirm device write",
            PlanItemId::new(2),
        )],
        failure_message: None,
    };
    PersistenceStore::record_event(
        &mut store,
        &sample_log(LogSeverity::Info).entry.attribution,
        &DomainEvent::VerificationReported(
            VerificationReport::warning("payload", "checksum mismatch"),
            snapshot,
        ),
    )
    .unwrap();

    let warnings = OperationLogRepository::list_logs(
        &store,
        &OperationLogQuery {
            execution_id: None,
            minimum_severity: Some(LogSeverity::Warning),
        },
    )
    .unwrap();
    assert!(warnings
        .iter()
        .any(|entry| entry.entry.message == "checksum mismatch"));
}

#[test]
fn preserves_foreign_keys_and_schema_bootstrap() {
    let store = SqliteStore::in_memory().unwrap();
    let error = store
        .connection()
        .execute(
            "INSERT INTO backup_manifest_entries (
                manifest_id, source_address, source_relative_path,
                backup_address, backup_relative_path, entry_kind, size_bytes, checksum_hex
             ) VALUES ('missing', 'local|L21udC9zb3VyY2U', 'source', 'logical|YmFja3Vw|ZmlsZQ', 'file', 'file', 1, NULL)",
            [],
        )
        .unwrap_err();

    assert!(matches!(error, rusqlite::Error::SqliteFailure(_, _)));
    assert_eq!(store.schema_version().unwrap(), SCHEMA_VERSION);
}

#[test]
fn rejects_non_absolute_manifest_source_roots() {
    let mut store = SqliteStore::in_memory().unwrap();
    let mut manifest = sample_manifest();
    manifest.source_root = Address::filesystem("relative/root");

    let error = ManifestRepository::save_manifest(&mut store, &manifest).unwrap_err();
    assert!(matches!(error, kc_domain::DomainError::Validation(_)));
}

#[test]
fn rejects_manifest_entries_outside_source_root_on_save() {
    let mut store = SqliteStore::in_memory().unwrap();
    let mut manifest = sample_manifest();
    manifest.entries[0].source = Address::filesystem("/mnt/other/settings.reader.lua");

    let error = store.save_manifest_internal(&manifest).unwrap_err();
    assert!(matches!(
        error,
        PersistenceError::Domain(_) | PersistenceError::InvalidPath { .. }
    ));
}

#[test]
fn rejects_invalid_persisted_manifest_relative_paths_on_load() {
    let mut store = SqliteStore::in_memory().unwrap();
    ManifestRepository::save_manifest(&mut store, &sample_manifest()).unwrap();
    store
        .connection()
        .execute(
            "UPDATE backup_manifest_entries SET source_relative_path = '../escape' WHERE manifest_id = 'backup-1'",
            [],
        )
        .unwrap();

    let error = store.load_manifest_internal("backup-1").unwrap_err();
    assert!(matches!(
        error,
        PersistenceError::InvalidPath {
            field: "backup_manifest.entry.source_relative_path",
            ..
        }
    ));
}

#[test]
fn enum_and_target_parsing_fail_closed() {
    let mut store = SqliteStore::in_memory().unwrap();
    kc_domain::DeviceRecordRepository::upsert_device_record(
        &mut store,
        &KnownDeviceRecord {
            descriptor: DeviceDescriptor {
                id: "device-1".to_string(),
                kind: DeviceKind::Other("boox".to_string()),
                display_name: "Boox".to_string(),
                support_level: SupportLevel::Supported,
            },
            last_seen_at_unix: 1,
            last_host_id: None,
            last_address: None,
        },
    )
    .unwrap();
    store
        .connection()
        .execute("UPDATE devices SET kind = 'boox' WHERE id = 'device-1'", [])
        .unwrap();
    let error = store.get_device_internal("device-1").unwrap_err();
    assert!(matches!(
        error,
        PersistenceError::InvalidEnum {
            field: "device_kind",
            ..
        }
    ));

    store
        .connection()
        .execute(
            "INSERT INTO operation_logs (
                plan_id, plan_item_id, execution_id, operation_id,
                target_kind, target_value, severity, message, recorded_at_unix
             ) VALUES (1, 1, 1, 1, 'address', 'scoped|known:usb-mass-storage|a29ibw|Li4vZXNjYXBl', 'info', 'test', 1)",
            [],
        )
        .unwrap();
    let error = store
        .list_logs_internal(&OperationLogQuery::all())
        .unwrap_err();
    assert!(matches!(error, PersistenceError::InvalidTarget { .. }));
}

#[test]
fn release_cache_round_trips_artifacts() {
    let mut store = SqliteStore::in_memory().unwrap();
    let release = sample_release();

    ReleaseMetadataCache::put_release_metadata(&mut store, &release).unwrap();
    let loaded = ReleaseMetadataCache::get_release_metadata(&store, "stable-kobo")
        .unwrap()
        .unwrap();
    assert_eq!(loaded.artifacts.len(), 1);
    assert_eq!(loaded.artifacts[0].name, "koreader-kobo.zip");
    assert_eq!(
        loaded.channel,
        CachedReleaseChannel::Other("nightly-kobo".to_string())
    );
}

#[test]
fn manifest_round_trips_scoped_source_roots() {
    let mut store = SqliteStore::in_memory().unwrap();
    let manifest = BackupManifestRecord {
        source_root: Address::scoped(TransportKind::UsbMassStorage, "device", ".kobo").unwrap(),
        entries: vec![BackupManifestEntryRecord {
            source: Address::scoped(
                TransportKind::UsbMassStorage,
                "device",
                ".kobo/Kobo/Kobo eReader.conf",
            )
            .unwrap(),
            backup: Address::filesystem("/tmp/backups/kobo/Kobo eReader.conf"),
            kind: BackupEntryKind::File,
            size_bytes: 512,
            checksum_hex: Some("deadbeef".to_string()),
        }],
        ..sample_manifest()
    };

    ManifestRepository::save_manifest(&mut store, &manifest).unwrap();
    let loaded = ManifestRepository::load_manifest(&store, "backup-1")
        .unwrap()
        .unwrap();
    assert_eq!(
        loaded.entries[0].backup,
        Address::filesystem("/tmp/backups/kobo/Kobo eReader.conf")
    );
}

#[test]
fn log_queries_filter_by_execution_and_severity() {
    let mut store = SqliteStore::in_memory().unwrap();
    let mut info = sample_log(LogSeverity::Info);
    info.entry.attribution.execution_id = ExecutionId::new(1);
    let mut warning = sample_log(LogSeverity::Warning);
    warning.entry.attribution.execution_id = ExecutionId::new(2);
    OperationLogRepository::append_log(&mut store, &info).unwrap();
    OperationLogRepository::append_log(&mut store, &warning).unwrap();

    let logs = OperationLogRepository::list_logs(
        &store,
        &OperationLogQuery {
            execution_id: Some(ExecutionId::new(2)),
            minimum_severity: Some(LogSeverity::Warning),
        },
    )
    .unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].entry.attribution.execution_id, ExecutionId::new(2));
}

#[cfg(unix)]
#[test]
fn manifests_round_trip_non_utf8_local_paths_losslessly() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::path::PathBuf;

    let mut store = SqliteStore::in_memory().unwrap();
    let source_root = PathBuf::from(OsString::from_vec(vec![b'/', b'm', b'n', b't', b'/', 0xFF]));
    let source_entry = PathBuf::from(OsString::from_vec(vec![
        b'/', b'm', b'n', b't', b'/', 0xFF, b'/', b'f', b'i', b'l', b'e',
    ]));
    let manifest = BackupManifestRecord {
        source_root: Address::filesystem(source_root.clone()),
        entries: vec![BackupManifestEntryRecord {
            source: Address::filesystem(source_entry.clone()),
            backup: Address::filesystem("/tmp/backups/file"),
            kind: BackupEntryKind::File,
            size_bytes: 1,
            checksum_hex: None,
        }],
        ..sample_manifest()
    };

    ManifestRepository::save_manifest(&mut store, &manifest).unwrap();
    let loaded = ManifestRepository::load_manifest(&store, "backup-1")
        .unwrap()
        .unwrap();

    assert_eq!(loaded.source_root, Address::filesystem(source_root));
    assert_eq!(loaded.entries[0].source, Address::filesystem(source_entry));
}
