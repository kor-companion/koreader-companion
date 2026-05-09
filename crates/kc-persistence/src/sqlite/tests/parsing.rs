use kc_domain::{
    CachedReleaseChannel, DeviceDescriptor, DeviceKind, ExecutionId, KnownDeviceRecord,
    LogSeverity, OperationLogQuery, OperationLogRepository, ReleaseMetadataCache, SupportLevel,
};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

use crate::sqlite::store::SqliteStore;
use crate::PersistenceError;

use super::fixtures::{sample_log, sample_release};

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

#[test]
fn filtered_log_queries_ignore_invalid_rows_outside_requested_execution() {
    let mut store = SqliteStore::in_memory().unwrap();
    let mut valid = sample_log(LogSeverity::Warning);
    valid.entry.attribution.execution_id = ExecutionId::new(2);
    OperationLogRepository::append_log(&mut store, &valid).unwrap();

    store
        .connection()
        .execute(
            "INSERT INTO operation_logs (
                plan_id, plan_item_id, execution_id, operation_id,
                target_kind, target_value, severity, message, recorded_at_unix
             ) VALUES (1, 1, 1, 1, 'address', 'scoped|known:usb-mass-storage|a29ibw|Li4vZXNjYXBl', 'info', 'bad', 1)",
            [],
        )
        .unwrap();

    let logs = OperationLogRepository::list_logs(
        &store,
        &OperationLogQuery {
            execution_id: Some(ExecutionId::new(2)),
            minimum_severity: Some(LogSeverity::Info),
        },
    )
    .unwrap();

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].entry.attribution.execution_id, ExecutionId::new(2));
}

#[test]
fn log_queries_reject_execution_id_overflow() {
    let store = SqliteStore::in_memory().unwrap();

    let error = store
        .list_logs_internal(&OperationLogQuery {
            execution_id: Some(ExecutionId::new(u64::MAX)),
            minimum_severity: None,
        })
        .unwrap_err();

    assert!(matches!(error, PersistenceError::InvalidNumber(i64::MAX)));
}

#[test]
fn opening_future_schema_versions_fails_closed() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("kc-persistence-schema-{unique}.sqlite"));

    let conn = Connection::open(&path).unwrap();
    crate::sqlite::schema::enable_foreign_keys(&conn).unwrap();
    crate::sqlite::schema::bootstrap(&conn).unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO schema_migrations (version, applied_at) VALUES (?1, strftime('%s','now'))",
        [crate::sqlite::schema::SCHEMA_VERSION + 1],
    )
    .unwrap();
    drop(conn);

    let error = match SqliteStore::open(&path) {
        Ok(_) => panic!("expected future schema version to be rejected"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        PersistenceError::UnsupportedSchemaVersion { .. }
    ));

    fs::remove_file(path).unwrap();
}
