use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use kc_domain::{
    BackupManifestEntryRecord, BackupManifestRecord, CachedReleaseMetadata, DeviceRecordRepository,
    DomainEvent, ExecutionId, KnownDeviceRecord, LogAttribution, LogSeverity, ManifestRepository,
    OperationLogEntry, OperationLogQuery, OperationLogRepository, PersistenceStore,
    ReleaseArtifactRecord, ReleaseMetadataCache, StoredOperationLog, VerificationStatus,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};

use crate::codec::address::{encode_address, parse_address};
use crate::codec::enums::{
    encode_backup_entry_kind, encode_device_kind, encode_log_severity, encode_release_channel,
    encode_support_level, parse_backup_entry_kind, parse_device_kind, parse_log_severity,
    parse_release_channel, parse_support_level,
};
use crate::codec::target::{encode_target, parse_target};
use crate::sqlite::schema::{bootstrap, enable_foreign_keys, SCHEMA_VERSION};
use crate::validation::{
    derive_source_relative_path, validate_address_projection, validate_source_root_address,
};
use crate::PersistenceError;

pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, PersistenceError> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        enable_foreign_keys(&store.conn)?;
        bootstrap(&store.conn)?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self, PersistenceError> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        enable_foreign_keys(&store.conn)?;
        bootstrap(&store.conn)?;
        Ok(store)
    }

    pub fn schema_version(&self) -> Result<i64, PersistenceError> {
        Ok(self
            .conn
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
                row.get::<_, Option<i64>>(0)
            })?
            .unwrap_or(0))
    }

    #[cfg(test)]
    pub(crate) fn connection(&self) -> &Connection {
        &self.conn
    }

    pub(crate) fn append_log_internal(
        &mut self,
        entry: &StoredOperationLog,
    ) -> Result<(), PersistenceError> {
        let (target_kind, target_value) = encode_target(&entry.entry.attribution.target);
        self.conn.execute(
            "INSERT INTO operation_logs (
                plan_id, plan_item_id, execution_id, operation_id,
                target_kind, target_value, severity, message, recorded_at_unix
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                to_i64(entry.entry.attribution.plan_id.value())?,
                to_i64(entry.entry.attribution.plan_item_id.value())?,
                to_i64(entry.entry.attribution.execution_id.value())?,
                to_i64(entry.entry.attribution.operation_id.value())?,
                target_kind,
                target_value,
                encode_log_severity(entry.entry.severity),
                &entry.entry.message,
                entry.recorded_at_unix,
            ],
        )?;
        Ok(())
    }

    pub(crate) fn list_logs_internal(
        &self,
        query: &OperationLogQuery,
    ) -> Result<Vec<StoredOperationLog>, PersistenceError> {
        let mut stmt = self.conn.prepare(
            "SELECT plan_id, plan_item_id, execution_id, operation_id,
                    target_kind, target_value, severity, message, recorded_at_unix
             FROM operation_logs ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, i64>(8)?,
            ))
        })?;

        let mut logs = Vec::new();
        for row in rows {
            let (
                plan_id,
                plan_item_id,
                execution_id,
                operation_id,
                target_kind,
                target_value,
                severity,
                message,
                recorded_at_unix,
            ) = row?;
            let severity = parse_log_severity(&severity)?;
            let log = StoredOperationLog {
                entry: OperationLogEntry::new(
                    LogAttribution {
                        plan_id: kc_domain::PlanId::new(from_i64(plan_id)?),
                        plan_item_id: kc_domain::PlanItemId::new(from_i64(plan_item_id)?),
                        execution_id: ExecutionId::new(from_i64(execution_id)?),
                        operation_id: kc_domain::OperationId::new(from_i64(operation_id)?),
                        target: parse_target(&target_kind, &target_value)?,
                    },
                    severity,
                    message,
                ),
                recorded_at_unix,
            };

            if matches_execution(query.execution_id, log.entry.attribution.execution_id)
                && matches_minimum_severity(query.minimum_severity, severity)
            {
                logs.push(log);
            }
        }
        Ok(logs)
    }

    fn upsert_device_internal(
        &mut self,
        record: &KnownDeviceRecord,
    ) -> Result<(), PersistenceError> {
        self.conn.execute(
            "INSERT INTO devices (
                id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                 kind = excluded.kind,
                 display_name = excluded.display_name,
                 support_level = excluded.support_level,
                 last_seen_at_unix = excluded.last_seen_at_unix,
                 last_host_id = excluded.last_host_id,
                 last_address = excluded.last_address",
            params![
                &record.descriptor.id,
                encode_device_kind(&record.descriptor.kind),
                &record.descriptor.display_name,
                encode_support_level(record.descriptor.support_level),
                record.last_seen_at_unix,
                &record.last_host_id,
                record.last_address.as_ref().map(encode_address),
            ],
        )?;
        Ok(())
    }

    pub(crate) fn get_device_internal(
        &self,
        id: &str,
    ) -> Result<Option<KnownDeviceRecord>, PersistenceError> {
        self.conn
            .query_row(
                "SELECT id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address
                 FROM devices WHERE id = ?1",
                [id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                    ))
                },
            )
            .optional()?
            .map(
                |(id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address)| {
                    Ok(KnownDeviceRecord {
                        descriptor: kc_domain::DeviceDescriptor {
                            id,
                            kind: parse_device_kind(&kind)?,
                            display_name,
                            support_level: parse_support_level(&support_level)?,
                        },
                        last_seen_at_unix,
                        last_host_id,
                        last_address: last_address
                            .as_deref()
                            .map(|value| parse_address("device.last_address", value))
                            .transpose()?,
                    })
                },
            )
            .transpose()
    }

    pub(crate) fn save_manifest_internal(
        &mut self,
        manifest: &BackupManifestRecord,
    ) -> Result<(), PersistenceError> {
        validate_source_root_address(&manifest.source_root)?;
        validate_manifest_entries(manifest)?;

        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO backup_manifests (
                manifest_id, device_id, created_at_unix, profile, app_version, schema_version, source_root
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &manifest.manifest_id,
                &manifest.device_id,
                manifest.created_at_unix,
                &manifest.profile,
                &manifest.app_version,
                manifest.schema_version,
                encode_address(&manifest.source_root),
            ],
        )?;
        tx.execute(
            "DELETE FROM backup_manifest_entries WHERE manifest_id = ?1",
            [&manifest.manifest_id],
        )?;
        insert_manifest_entries(&tx, manifest)?;
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn load_manifest_internal(
        &self,
        id: &str,
    ) -> Result<Option<BackupManifestRecord>, PersistenceError> {
        let manifest = self
            .conn
            .query_row(
                "SELECT manifest_id, device_id, created_at_unix, profile, app_version, schema_version, source_root
                 FROM backup_manifests WHERE manifest_id = ?1",
                [id],
                |row| {
                    let source_root = row.get::<_, String>(6)?;
                    Ok(BackupManifestRecord {
                        manifest_id: row.get(0)?,
                        device_id: row.get(1)?,
                        created_at_unix: row.get(2)?,
                        profile: row.get(3)?,
                        app_version: row.get(4)?,
                        schema_version: row.get(5)?,
                        source_root: parse_address("backup_manifest.source_root", &source_root)
                            .map_err(|error| rusqlite::Error::FromSqlConversionFailure(
                                6,
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            ))?,
                        entries: Vec::new(),
                    })
                },
            )
            .optional()?;

        let Some(mut manifest) = manifest else {
            return Ok(None);
        };
        validate_source_root_address(&manifest.source_root)?;

        let mut stmt = self.conn.prepare(
            "SELECT source_address, source_relative_path, backup_address, backup_relative_path,
                    entry_kind, size_bytes, checksum_hex
             FROM backup_manifest_entries WHERE manifest_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, Option<String>>(6)?,
            ))
        })?;

        let mut entries = Vec::new();
        for row in rows {
            let (
                source_address,
                source_relative_path,
                backup_address,
                backup_relative_path,
                entry_kind,
                size_bytes,
                checksum_hex,
            ) = row?;
            let source = parse_address("backup_manifest.entry.source", &source_address)?;
            let backup = parse_address("backup_manifest.entry.backup", &backup_address)?;
            let expected_source_relative =
                derive_source_relative_path(&manifest.source_root, &source)?;
            let expected_backup_relative =
                validate_address_projection("backup_manifest.entry.backup", &backup)?;
            if source_relative_path != expected_source_relative {
                return Err(PersistenceError::InvalidPath {
                    field: "backup_manifest.entry.source_relative_path",
                    value: source_relative_path,
                });
            }
            if backup_relative_path != expected_backup_relative {
                return Err(PersistenceError::InvalidPath {
                    field: "backup_manifest.entry.backup_relative_path",
                    value: backup_relative_path,
                });
            }
            entries.push(BackupManifestEntryRecord {
                source,
                backup,
                kind: parse_backup_entry_kind(&entry_kind)?,
                size_bytes: from_i64(size_bytes)?,
                checksum_hex,
            });
        }

        manifest.entries = entries;
        Ok(Some(manifest))
    }

    fn put_release_internal(
        &mut self,
        release: &CachedReleaseMetadata,
    ) -> Result<(), PersistenceError> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO release_metadata_cache (
                cache_key, release_id, version, channel, published_at_unix, fetched_at_unix, source_url
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &release.cache_key,
                &release.release_id,
                &release.version,
                encode_release_channel(&release.channel),
                release.published_at_unix,
                release.fetched_at_unix,
                &release.source_url,
            ],
        )?;
        tx.execute(
            "DELETE FROM release_artifacts WHERE cache_key = ?1",
            [&release.cache_key],
        )?;
        for artifact in &release.artifacts {
            tx.execute(
                "INSERT INTO release_artifacts (
                    cache_key, name, download_url, size_bytes, content_type, checksum_hex
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    &release.cache_key,
                    &artifact.name,
                    &artifact.download_url,
                    to_i64(artifact.size_bytes)?,
                    &artifact.content_type,
                    &artifact.checksum_hex,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn get_release_internal(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, PersistenceError> {
        let release = self
            .conn
            .query_row(
                "SELECT cache_key, release_id, version, channel, published_at_unix, fetched_at_unix, source_url
                 FROM release_metadata_cache WHERE cache_key = ?1",
                [cache_key],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, String>(6)?,
                    ))
                },
            )
            .optional()?;

        let Some((
            cache_key,
            release_id,
            version,
            channel,
            published_at_unix,
            fetched_at_unix,
            source_url,
        )) = release
        else {
            return Ok(None);
        };

        let mut stmt = self.conn.prepare(
            "SELECT name, download_url, size_bytes, content_type, checksum_hex
             FROM release_artifacts WHERE cache_key = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([cache_key.clone()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut artifacts = Vec::new();
        for row in rows {
            let (name, download_url, size_bytes, content_type, checksum_hex) = row?;
            artifacts.push(ReleaseArtifactRecord {
                name,
                download_url,
                size_bytes: from_i64(size_bytes)?,
                content_type,
                checksum_hex,
            });
        }
        Ok(Some(CachedReleaseMetadata {
            cache_key,
            release_id,
            version,
            channel: parse_release_channel(&channel)?,
            published_at_unix,
            fetched_at_unix,
            source_url,
            artifacts,
        }))
    }
}

impl OperationLogRepository for SqliteStore {
    fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), kc_domain::DomainError> {
        self.append_log_internal(entry).map_err(Into::into)
    }

    fn list_logs(
        &self,
        query: &OperationLogQuery,
    ) -> Result<Vec<StoredOperationLog>, kc_domain::DomainError> {
        self.list_logs_internal(query).map_err(Into::into)
    }
}

impl DeviceRecordRepository for SqliteStore {
    fn upsert_device_record(
        &mut self,
        record: &KnownDeviceRecord,
    ) -> Result<(), kc_domain::DomainError> {
        self.upsert_device_internal(record).map_err(Into::into)
    }

    fn get_device_record(
        &self,
        id: &str,
    ) -> Result<Option<KnownDeviceRecord>, kc_domain::DomainError> {
        self.get_device_internal(id).map_err(Into::into)
    }
}

impl ManifestRepository for SqliteStore {
    fn save_manifest(
        &mut self,
        manifest: &BackupManifestRecord,
    ) -> Result<(), kc_domain::DomainError> {
        self.save_manifest_internal(manifest).map_err(Into::into)
    }

    fn load_manifest(
        &self,
        id: &str,
    ) -> Result<Option<BackupManifestRecord>, kc_domain::DomainError> {
        self.load_manifest_internal(id).map_err(Into::into)
    }
}

impl ReleaseMetadataCache for SqliteStore {
    fn put_release_metadata(
        &mut self,
        release: &CachedReleaseMetadata,
    ) -> Result<(), kc_domain::DomainError> {
        self.put_release_internal(release).map_err(Into::into)
    }

    fn get_release_metadata(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, kc_domain::DomainError> {
        self.get_release_internal(cache_key).map_err(Into::into)
    }
}

impl PersistenceStore for SqliteStore {
    fn record_event(
        &mut self,
        attribution: &LogAttribution,
        event: &DomainEvent,
    ) -> Result<(), kc_domain::DomainError> {
        let (severity, message) = describe_event(event);
        self.append_log(&StoredOperationLog {
            entry: OperationLogEntry::new(attribution.clone(), severity, message),
            recorded_at_unix: unix_now(),
        })
    }
}

fn insert_manifest_entries(
    tx: &Transaction<'_>,
    manifest: &BackupManifestRecord,
) -> Result<(), PersistenceError> {
    for entry in &manifest.entries {
        tx.execute(
            "INSERT INTO backup_manifest_entries (
                manifest_id, source_address, source_relative_path,
                backup_address, backup_relative_path, entry_kind, size_bytes, checksum_hex
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                manifest.manifest_id,
                encode_address(&entry.source),
                derive_source_relative_path(&manifest.source_root, &entry.source)?,
                encode_address(&entry.backup),
                validate_address_projection("backup_manifest.entry.backup", &entry.backup)?,
                encode_backup_entry_kind(entry.kind),
                to_i64(entry.size_bytes)?,
                entry.checksum_hex,
            ],
        )?;
    }
    Ok(())
}

fn validate_manifest_entries(manifest: &BackupManifestRecord) -> Result<(), PersistenceError> {
    for entry in &manifest.entries {
        derive_source_relative_path(&manifest.source_root, &entry.source)?;
        validate_address_projection("backup_manifest.entry.backup", &entry.backup)?;
    }
    Ok(())
}

fn describe_event(event: &DomainEvent) -> (LogSeverity, String) {
    match event {
        DomainEvent::WorkflowPlanned(snapshot) => (
            LogSeverity::Info,
            format!("workflow planned in {:?} mode", snapshot.mode),
        ),
        DomainEvent::ConfirmationRequired(gate, _) => (
            LogSeverity::Warning,
            format!("confirmation required: {}", gate.message),
        ),
        DomainEvent::PhaseChanged(phase, _) => (
            phase_severity(*phase),
            format!("workflow phase changed to {phase:?}"),
        ),
        DomainEvent::ProgressUpdated(update, _) => (
            LogSeverity::Info,
            update.message.clone().unwrap_or_else(|| {
                format!("progress {}/{}", update.completed_items, update.total_items)
            }),
        ),
        DomainEvent::VerificationReported(report, _) => {
            let summary = report
                .items
                .first()
                .and_then(|item| item.message.clone())
                .unwrap_or_else(|| "verification report recorded".to_string());
            (verification_severity(report.status), summary)
        }
        DomainEvent::PlanItemStarted(id, _) => (
            LogSeverity::Info,
            format!("plan item {} started", id.value()),
        ),
        DomainEvent::PlanItemCompleted(id, _) => (
            LogSeverity::Info,
            format!("plan item {} completed", id.value()),
        ),
        DomainEvent::WorkflowFinished(snapshot) => (
            phase_severity(snapshot.phase),
            format!("workflow finished in {:?}", snapshot.phase),
        ),
    }
}

fn phase_severity(phase: kc_domain::WorkflowPhase) -> LogSeverity {
    match phase {
        kc_domain::WorkflowPhase::Failed => LogSeverity::Error,
        kc_domain::WorkflowPhase::Cancelled | kc_domain::WorkflowPhase::AwaitingConfirmation => {
            LogSeverity::Warning
        }
        kc_domain::WorkflowPhase::Planned
        | kc_domain::WorkflowPhase::Ready
        | kc_domain::WorkflowPhase::Running
        | kc_domain::WorkflowPhase::Succeeded => LogSeverity::Info,
    }
}

fn verification_severity(status: VerificationStatus) -> LogSeverity {
    match status {
        VerificationStatus::Failed => LogSeverity::Error,
        VerificationStatus::Warning => LogSeverity::Warning,
        VerificationStatus::Pending | VerificationStatus::Passed => LogSeverity::Info,
    }
}

fn matches_execution(query: Option<ExecutionId>, value: ExecutionId) -> bool {
    query.map(|expected| expected == value).unwrap_or(true)
}

fn matches_minimum_severity(query: Option<LogSeverity>, value: LogSeverity) -> bool {
    query
        .map(|minimum| severity_rank(value) >= severity_rank(minimum))
        .unwrap_or(true)
}

fn severity_rank(value: LogSeverity) -> u8 {
    match value {
        LogSeverity::Info => 1,
        LogSeverity::Warning => 2,
        LogSeverity::Error => 3,
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn to_i64(value: u64) -> Result<i64, PersistenceError> {
    i64::try_from(value).map_err(|_| PersistenceError::InvalidNumber(i64::MAX))
}

fn from_i64(value: i64) -> Result<u64, PersistenceError> {
    u64::try_from(value).map_err(|_| PersistenceError::InvalidNumber(value))
}

#[allow(dead_code)]
pub(crate) const fn schema_version() -> i64 {
    SCHEMA_VERSION
}
