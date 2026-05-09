use kc_domain::{
    ConfirmationGate, ConfirmationId, DeviceDescriptor, DeviceKind, DomainEvent, ExecutionId,
    ExecutionMode, KnownDeviceRecord, LogSeverity, ManifestRepository, OperationLogQuery,
    OperationLogRepository, PersistenceStore, PlanId, PlanItemId, ReleaseMetadataCache,
    SupportLevel, VerificationReport, WorkflowKind, WorkflowPhase,
};

use crate::sqlite::schema::SCHEMA_VERSION;
use crate::sqlite::store::SqliteStore;

use super::fixtures::{sample_log, sample_manifest, sample_release};

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
            kc_domain::Address::scoped(
                kc_domain::TransportKind::UsbMassStorage,
                "kobo-usb",
                ".kobo",
            )
            .unwrap(),
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
