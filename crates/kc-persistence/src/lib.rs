use std::error::Error;
use std::fmt;
use std::path::Path;

use kc_domain::{
    ContainmentPolicy, DeviceDescriptor, DeviceKind, ExecutionId, LogAttribution, LogSeverity,
    OperationId, OperationLogEntry, OperationTarget, PlanId, PlanItemId, SupportLevel,
};
use kc_payload::{
    Checksum, ChecksumAlgorithm, ReleaseAsset, ReleaseChannel, ReleaseMetadata, Timestamp,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};

pub const SCHEMA_VERSION: i64 = 1;

pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, PersistenceError> {
        let conn = Connection::open(path)?;
        let mut store = Self { conn };
        store.enable_foreign_keys()?;
        store.bootstrap()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self, PersistenceError> {
        let conn = Connection::open_in_memory()?;
        let mut store = Self { conn };
        store.enable_foreign_keys()?;
        store.bootstrap()?;
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

    pub fn append_log(&mut self, record: &StoredLogEntry) -> Result<i64, PersistenceError> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO operation_logs (
                plan_id, plan_item_id, execution_id, operation_id,
                target_kind, target_value, severity, message, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                to_i64(record.entry.attribution.plan_id.value())?,
                to_i64(record.entry.attribution.plan_item_id.value())?,
                to_i64(record.entry.attribution.execution_id.value())?,
                to_i64(record.entry.attribution.operation_id.value())?,
                target_kind(&record.entry.attribution.target),
                target_value(&record.entry.attribution.target),
                severity_text(record.entry.severity),
                record.entry.message,
                record.created_at.unix_seconds(),
            ],
        )?;
        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(id)
    }

    pub fn list_logs(&self) -> Result<Vec<StoredLogEntry>, PersistenceError> {
        let mut stmt = self.conn.prepare(
            "SELECT plan_id, plan_item_id, execution_id, operation_id,
                    target_kind, target_value, severity, message, created_at
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

        let raw = rows.collect::<Result<Vec<_>, _>>()?;
        raw.into_iter()
            .map(
                |(
                    plan_id,
                    plan_item_id,
                    execution_id,
                    operation_id,
                    target_kind,
                    target_value,
                    severity,
                    message,
                    created_at,
                )| {
                    Ok(StoredLogEntry {
                        entry: OperationLogEntry {
                            attribution: LogAttribution {
                                plan_id: PlanId::new(from_i64(plan_id)?),
                                plan_item_id: PlanItemId::new(from_i64(plan_item_id)?),
                                execution_id: ExecutionId::new(from_i64(execution_id)?),
                                operation_id: OperationId::new(from_i64(operation_id)?),
                                target: parse_target(target_kind, target_value)?,
                            },
                            severity: parse_severity(severity)?,
                            message,
                        },
                        created_at: Timestamp::from_unix_seconds(created_at),
                    })
                },
            )
            .collect()
    }

    pub fn upsert_device(&mut self, record: &DeviceRecord) -> Result<(), PersistenceError> {
        self.conn.execute(
            "INSERT INTO devices (id, kind, display_name, support_level, last_seen_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET
                 kind = excluded.kind,
                 display_name = excluded.display_name,
                 support_level = excluded.support_level,
                 last_seen_at = excluded.last_seen_at",
            params![
                record.descriptor.id,
                device_kind_text(&record.descriptor.kind),
                record.descriptor.display_name,
                support_level_text(record.descriptor.support_level),
                record.last_seen_at.unix_seconds(),
            ],
        )?;
        Ok(())
    }

    pub fn get_device(&self, id: &str) -> Result<Option<DeviceRecord>, PersistenceError> {
        let record = self
            .conn
            .query_row(
                "SELECT id, kind, display_name, support_level, last_seen_at FROM devices WHERE id = ?1",
                [id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                    ))
                },
            )
            .optional()?;

        record
            .map(|(id, kind, display_name, support_level, last_seen_at)| {
                Ok(DeviceRecord {
                    descriptor: DeviceDescriptor {
                        id,
                        kind: parse_device_kind(kind)?,
                        display_name,
                        support_level: parse_support_level(support_level)?,
                    },
                    last_seen_at: Timestamp::from_unix_seconds(last_seen_at),
                })
            })
            .transpose()
    }

    pub fn save_backup_manifest(
        &mut self,
        manifest: &BackupManifest,
    ) -> Result<(), PersistenceError> {
        validate_absolute_path_field("backup_manifest.source_root", &manifest.source_root)?;
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO backup_manifests (
                backup_id, device_id, created_at, profile, app_version, schema_version, source_root
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                manifest.backup_id,
                manifest.device_id,
                manifest.created_at.unix_seconds(),
                manifest.profile,
                manifest.app_version,
                manifest.schema_version,
                manifest.source_root,
            ],
        )?;
        tx.execute(
            "DELETE FROM backup_manifest_entries WHERE backup_id = ?1",
            [&manifest.backup_id],
        )?;
        insert_manifest_entries(&tx, manifest)?;
        tx.commit()?;
        Ok(())
    }

    pub fn load_backup_manifest(
        &self,
        backup_id: &str,
    ) -> Result<Option<BackupManifest>, PersistenceError> {
        let manifest = self
            .conn
            .query_row(
                "SELECT backup_id, device_id, created_at, profile, app_version, schema_version, source_root
                 FROM backup_manifests WHERE backup_id = ?1",
                [backup_id],
                |row| {
                    Ok(BackupManifest {
                        backup_id: row.get(0)?,
                        device_id: row.get(1)?,
                        created_at: Timestamp::from_unix_seconds(row.get(2)?),
                        profile: row.get(3)?,
                        app_version: row.get(4)?,
                        schema_version: row.get(5)?,
                        source_root: row.get(6)?,
                        entries: Vec::new(),
                    })
                },
            )
            .optional()?;

        let Some(mut manifest) = manifest else {
            return Ok(None);
        };
        validate_absolute_path_field("backup_manifest.source_root", &manifest.source_root)?;

        let mut stmt = self.conn.prepare(
            "SELECT source_relative_path, backup_relative_path, entry_kind, size_bytes,
                    modified_at, checksum_algorithm, checksum_hex, status
             FROM backup_manifest_entries WHERE backup_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([backup_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;
        let raw = rows.collect::<Result<Vec<_>, _>>()?;
        manifest.entries = raw
            .into_iter()
            .map(
                |(
                    source_relative_path,
                    backup_relative_path,
                    entry_kind,
                    size_bytes,
                    modified_at,
                    checksum_algorithm,
                    checksum_hex,
                    status,
                )| {
                    Ok(BackupManifestEntry {
                        source_relative_path,
                        backup_relative_path,
                        entry_kind: parse_manifest_entry_kind(entry_kind)?,
                        size_bytes: from_i64(size_bytes)?,
                        modified_at: Timestamp::from_unix_seconds(modified_at),
                        checksum: parse_checksum_parts(checksum_algorithm, checksum_hex)?,
                        status: parse_manifest_entry_status(status)?,
                    })
                },
            )
            .collect::<Result<Vec<_>, PersistenceError>>()?;
        Ok(Some(manifest))
    }

    pub fn put_release_metadata(
        &mut self,
        release: &ReleaseMetadata,
    ) -> Result<(), PersistenceError> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO release_metadata_cache (
                release_id, version, channel, published_at, fetched_at, source_url
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                release.release_id,
                release.version,
                release_channel_text(release.channel),
                release.published_at.unix_seconds(),
                release.fetched_at.unix_seconds(),
                release.source_url,
            ],
        )?;
        tx.execute(
            "DELETE FROM release_assets WHERE release_id = ?1",
            [&release.release_id],
        )?;
        for asset in &release.assets {
            tx.execute(
                "INSERT INTO release_assets (
                    release_id, name, download_url, size_bytes, content_type, checksum_algorithm, checksum_hex
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    release.release_id,
                    asset.name,
                    asset.download_url,
                    to_i64(asset.size_bytes)?,
                    asset.content_type,
                    asset.checksum.as_ref().map(|value| checksum_algorithm_text(value.algorithm())),
                    asset.checksum.as_ref().map(|value| value.hex().to_string()),
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_release_metadata(
        &self,
        release_id: &str,
    ) -> Result<Option<ReleaseMetadata>, PersistenceError> {
        let release = self
            .conn
            .query_row(
                "SELECT release_id, version, channel, published_at, fetched_at, source_url
                 FROM release_metadata_cache WHERE release_id = ?1",
                [release_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )
            .optional()?;

        let Some((release_id, version, channel, published_at, fetched_at, source_url)) = release
        else {
            return Ok(None);
        };
        let release_id_for_assets = release_id.clone();

        let mut release = ReleaseMetadata {
            release_id,
            version,
            channel: parse_release_channel(channel)?,
            published_at: Timestamp::from_unix_seconds(published_at),
            fetched_at: Timestamp::from_unix_seconds(fetched_at),
            source_url,
            assets: Vec::new(),
        };

        let mut stmt = self.conn.prepare(
            "SELECT name, download_url, size_bytes, content_type, checksum_algorithm, checksum_hex
             FROM release_assets WHERE release_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([release_id_for_assets], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
            ))
        })?;
        let raw = rows.collect::<Result<Vec<_>, _>>()?;
        release.assets = raw
            .into_iter()
            .map(
                |(
                    name,
                    download_url,
                    size_bytes,
                    content_type,
                    checksum_algorithm,
                    checksum_hex,
                )| {
                    Ok(ReleaseAsset {
                        name,
                        download_url,
                        size_bytes: from_i64(size_bytes)?,
                        content_type,
                        checksum: parse_checksum_parts(checksum_algorithm, checksum_hex)?,
                    })
                },
            )
            .collect::<Result<Vec<_>, PersistenceError>>()?;
        Ok(Some(release))
    }

    fn bootstrap(&mut self) -> Result<(), PersistenceError> {
        self.conn.execute_batch(
            "BEGIN;
             CREATE TABLE IF NOT EXISTS schema_migrations (
                 version INTEGER PRIMARY KEY,
                 applied_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS operation_logs (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 plan_id INTEGER NOT NULL,
                 plan_item_id INTEGER NOT NULL,
                 execution_id INTEGER NOT NULL,
                 operation_id INTEGER NOT NULL,
                 target_kind TEXT NOT NULL,
                 target_value TEXT NOT NULL,
                 severity TEXT NOT NULL,
                 message TEXT NOT NULL,
                 created_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS devices (
                 id TEXT PRIMARY KEY,
                 kind TEXT NOT NULL,
                 display_name TEXT NOT NULL,
                 support_level TEXT NOT NULL,
                 last_seen_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS backup_manifests (
                 backup_id TEXT PRIMARY KEY,
                 device_id TEXT NOT NULL,
                 created_at INTEGER NOT NULL,
                 profile TEXT NOT NULL,
                 app_version TEXT NOT NULL,
                 schema_version INTEGER NOT NULL,
                 source_root TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS backup_manifest_entries (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 backup_id TEXT NOT NULL,
                 source_relative_path TEXT NOT NULL,
                 backup_relative_path TEXT NOT NULL,
                 entry_kind TEXT NOT NULL,
                 size_bytes INTEGER NOT NULL,
                 modified_at INTEGER NOT NULL,
                 checksum_algorithm TEXT,
                 checksum_hex TEXT,
                 status TEXT NOT NULL,
                 FOREIGN KEY(backup_id) REFERENCES backup_manifests(backup_id) ON DELETE CASCADE
             );
             CREATE TABLE IF NOT EXISTS release_metadata_cache (
                 release_id TEXT PRIMARY KEY,
                 version TEXT NOT NULL,
                 channel TEXT NOT NULL,
                 published_at INTEGER NOT NULL,
                 fetched_at INTEGER NOT NULL,
                 source_url TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS release_assets (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 release_id TEXT NOT NULL,
                 name TEXT NOT NULL,
                 download_url TEXT NOT NULL,
                 size_bytes INTEGER NOT NULL,
                 content_type TEXT,
                 checksum_algorithm TEXT,
                 checksum_hex TEXT,
                 FOREIGN KEY(release_id) REFERENCES release_metadata_cache(release_id) ON DELETE CASCADE
             );
             COMMIT;",
        )?;
        self.conn.execute(
            "INSERT OR IGNORE INTO schema_migrations (version, applied_at) VALUES (?1, strftime('%s','now'))",
            [SCHEMA_VERSION],
        )?;
        Ok(())
    }

    fn enable_foreign_keys(&mut self) -> Result<(), PersistenceError> {
        self.conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredLogEntry {
    pub entry: OperationLogEntry,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRecord {
    pub descriptor: DeviceDescriptor,
    pub last_seen_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestEntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestEntryStatus {
    Copied,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifestEntry {
    pub source_relative_path: String,
    pub backup_relative_path: String,
    pub entry_kind: ManifestEntryKind,
    pub size_bytes: u64,
    pub modified_at: Timestamp,
    pub checksum: Option<Checksum>,
    pub status: ManifestEntryStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifest {
    pub backup_id: String,
    pub device_id: String,
    pub created_at: Timestamp,
    pub profile: String,
    pub app_version: String,
    pub schema_version: i64,
    pub source_root: String,
    pub entries: Vec<BackupManifestEntry>,
}

#[derive(Debug)]
pub enum PersistenceError {
    Sqlite(rusqlite::Error),
    InvalidNumber(i64),
    InvalidEnum { field: &'static str, value: String },
    InvalidChecksum(String),
    InvalidPath { field: &'static str, value: String },
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "sqlite error: {error}"),
            Self::InvalidNumber(value) => write!(f, "invalid stored integer value: {value}"),
            Self::InvalidEnum { field, value } => write!(f, "invalid {field} value: {value}"),
            Self::InvalidChecksum(value) => write!(f, "invalid checksum value: {value}"),
            Self::InvalidPath { field, value } => write!(f, "invalid {field} path: {value}"),
        }
    }
}

impl Error for PersistenceError {}

impl From<rusqlite::Error> for PersistenceError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

fn insert_manifest_entries(
    tx: &Transaction<'_>,
    manifest: &BackupManifest,
) -> Result<(), PersistenceError> {
    for entry in &manifest.entries {
        tx.execute(
            "INSERT INTO backup_manifest_entries (
                backup_id, source_relative_path, backup_relative_path, entry_kind,
                size_bytes, modified_at, checksum_algorithm, checksum_hex, status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                manifest.backup_id,
                entry.source_relative_path,
                entry.backup_relative_path,
                manifest_entry_kind_text(entry.entry_kind),
                to_i64(entry.size_bytes)?,
                entry.modified_at.unix_seconds(),
                entry
                    .checksum
                    .as_ref()
                    .map(|value| checksum_algorithm_text(value.algorithm())),
                entry.checksum.as_ref().map(|value| value.hex().to_string()),
                manifest_entry_status_text(entry.status),
            ],
        )?;
    }
    Ok(())
}

fn to_i64(value: u64) -> Result<i64, PersistenceError> {
    i64::try_from(value).map_err(|_| PersistenceError::InvalidNumber(i64::MAX))
}

fn from_i64(value: i64) -> Result<u64, PersistenceError> {
    u64::try_from(value).map_err(|_| PersistenceError::InvalidNumber(value))
}

fn target_kind(target: &OperationTarget) -> &'static str {
    match target {
        OperationTarget::HostPath(_) => "host_path",
        OperationTarget::DevicePath(_) => "device_path",
        OperationTarget::Payload(_) => "payload",
        OperationTarget::Logical(_) => "logical",
    }
}

fn target_value(target: &OperationTarget) -> String {
    match target {
        OperationTarget::HostPath(path) | OperationTarget::DevicePath(path) => {
            path.display().to_string()
        }
        OperationTarget::Payload(value) | OperationTarget::Logical(value) => value.clone(),
    }
}

fn parse_target(kind: String, value: String) -> Result<OperationTarget, PersistenceError> {
    match kind.as_str() {
        "host_path" => Ok(OperationTarget::HostPath(value.into())),
        "device_path" => Ok(OperationTarget::DevicePath(value.into())),
        "payload" => Ok(OperationTarget::Payload(value)),
        "logical" => Ok(OperationTarget::Logical(value)),
        _ => Err(PersistenceError::InvalidEnum {
            field: "target_kind",
            value: kind,
        }),
    }
}

fn severity_text(value: LogSeverity) -> &'static str {
    match value {
        LogSeverity::Info => "info",
        LogSeverity::Warning => "warning",
        LogSeverity::Error => "error",
    }
}

fn parse_severity(value: String) -> Result<LogSeverity, PersistenceError> {
    match value.as_str() {
        "info" => Ok(LogSeverity::Info),
        "warning" => Ok(LogSeverity::Warning),
        "error" => Ok(LogSeverity::Error),
        _ => Err(PersistenceError::InvalidEnum {
            field: "severity",
            value,
        }),
    }
}

fn device_kind_text(value: &DeviceKind) -> &str {
    match value {
        DeviceKind::Kobo => "kobo",
        DeviceKind::PocketBook => "pocketbook",
        DeviceKind::Kindle => "kindle",
        DeviceKind::Android => "android",
        DeviceKind::Remarkable => "remarkable",
        DeviceKind::Other(value) => value.as_str(),
    }
}

fn parse_device_kind(value: String) -> Result<DeviceKind, PersistenceError> {
    match value.as_str() {
        "kobo" => Ok(DeviceKind::Kobo),
        "pocketbook" => Ok(DeviceKind::PocketBook),
        "kindle" => Ok(DeviceKind::Kindle),
        "android" => Ok(DeviceKind::Android),
        "remarkable" => Ok(DeviceKind::Remarkable),
        other if other.is_empty() => Err(PersistenceError::InvalidEnum {
            field: "device_kind",
            value,
        }),
        other => Ok(DeviceKind::Other(other.to_string())),
    }
}

fn support_level_text(value: SupportLevel) -> &'static str {
    match value {
        SupportLevel::Supported => "supported",
        SupportLevel::Experimental => "experimental",
        SupportLevel::Unsupported => "unsupported",
    }
}

fn parse_support_level(value: String) -> Result<SupportLevel, PersistenceError> {
    match value.as_str() {
        "supported" => Ok(SupportLevel::Supported),
        "experimental" => Ok(SupportLevel::Experimental),
        "unsupported" => Ok(SupportLevel::Unsupported),
        _ => Err(PersistenceError::InvalidEnum {
            field: "support_level",
            value,
        }),
    }
}

fn manifest_entry_kind_text(value: ManifestEntryKind) -> &'static str {
    match value {
        ManifestEntryKind::File => "file",
        ManifestEntryKind::Directory => "directory",
    }
}

fn parse_manifest_entry_kind(value: String) -> Result<ManifestEntryKind, PersistenceError> {
    match value.as_str() {
        "file" => Ok(ManifestEntryKind::File),
        "directory" => Ok(ManifestEntryKind::Directory),
        _ => Err(PersistenceError::InvalidEnum {
            field: "manifest_entry_kind",
            value,
        }),
    }
}

fn manifest_entry_status_text(value: ManifestEntryStatus) -> &'static str {
    match value {
        ManifestEntryStatus::Copied => "copied",
        ManifestEntryStatus::Skipped => "skipped",
    }
}

fn parse_manifest_entry_status(value: String) -> Result<ManifestEntryStatus, PersistenceError> {
    match value.as_str() {
        "copied" => Ok(ManifestEntryStatus::Copied),
        "skipped" => Ok(ManifestEntryStatus::Skipped),
        _ => Err(PersistenceError::InvalidEnum {
            field: "manifest_entry_status",
            value,
        }),
    }
}

fn release_channel_text(value: ReleaseChannel) -> &'static str {
    match value {
        ReleaseChannel::Stable => "stable",
        ReleaseChannel::Prerelease => "prerelease",
    }
}

fn parse_release_channel(value: String) -> Result<ReleaseChannel, PersistenceError> {
    match value.as_str() {
        "stable" => Ok(ReleaseChannel::Stable),
        "prerelease" => Ok(ReleaseChannel::Prerelease),
        _ => Err(PersistenceError::InvalidEnum {
            field: "release_channel",
            value,
        }),
    }
}

fn checksum_algorithm_text(value: ChecksumAlgorithm) -> &'static str {
    match value {
        ChecksumAlgorithm::Sha256 => "sha256",
        ChecksumAlgorithm::Sha512 => "sha512",
    }
}

fn parse_checksum_parts(
    algorithm: Option<String>,
    hex: Option<String>,
) -> Result<Option<Checksum>, PersistenceError> {
    match (algorithm, hex) {
        (Some(algorithm), Some(hex)) => {
            let algorithm = match algorithm.as_str() {
                "sha256" => ChecksumAlgorithm::Sha256,
                "sha512" => ChecksumAlgorithm::Sha512,
                _ => {
                    return Err(PersistenceError::InvalidEnum {
                        field: "checksum_algorithm",
                        value: algorithm,
                    })
                }
            };
            Checksum::new(algorithm, hex)
                .map(Some)
                .map_err(|error| PersistenceError::InvalidChecksum(error.to_string()))
        }
        (None, None) => Ok(None),
        (algorithm, hex) => Err(PersistenceError::InvalidChecksum(format!(
            "mismatched checksum parts: {algorithm:?} {hex:?}"
        ))),
    }
}

fn validate_absolute_path_field(field: &'static str, value: &str) -> Result<(), PersistenceError> {
    let path = Path::new(value);
    if !path.is_absolute() {
        return Err(PersistenceError::InvalidPath {
            field,
            value: value.to_string(),
        });
    }

    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(PersistenceError::InvalidPath {
            field,
            value: value.to_string(),
        });
    }

    ContainmentPolicy::new(path)
        .map(|_| ())
        .map_err(|_| PersistenceError::InvalidPath {
            field,
            value: value.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kc_domain::{DeviceKind, OperationTarget};

    fn checksum() -> Checksum {
        Checksum::new(
            ChecksumAlgorithm::Sha256,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap()
    }

    #[test]
    fn bootstraps_schema_version() {
        let store = SqliteStore::in_memory().unwrap();
        assert_eq!(store.schema_version().unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn persists_operation_logs() {
        let mut store = SqliteStore::in_memory().unwrap();
        store
            .append_log(&StoredLogEntry {
                entry: OperationLogEntry::new(
                    LogAttribution {
                        plan_id: PlanId::new(10),
                        plan_item_id: PlanItemId::new(2),
                        execution_id: ExecutionId::new(4),
                        operation_id: OperationId::new(8),
                        target: OperationTarget::DevicePath("/.adds/koreader".into()),
                    },
                    LogSeverity::Info,
                    "staged payload",
                ),
                created_at: Timestamp::from_unix_seconds(1_713_000_123),
            })
            .unwrap();

        let logs = store.list_logs().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].entry.message, "staged payload");
        assert_eq!(logs[0].created_at.unix_seconds(), 1_713_000_123);
    }

    #[test]
    fn persists_device_records() {
        let mut store = SqliteStore::in_memory().unwrap();
        store
            .upsert_device(&DeviceRecord {
                descriptor: DeviceDescriptor {
                    id: "device-1".to_string(),
                    kind: DeviceKind::Kobo,
                    display_name: "Reader".to_string(),
                    support_level: SupportLevel::Supported,
                },
                last_seen_at: Timestamp::from_unix_seconds(1_713_000_200),
            })
            .unwrap();

        let device = store.get_device("device-1").unwrap().unwrap();
        assert_eq!(device.descriptor.display_name, "Reader");
        assert_eq!(device.last_seen_at.unix_seconds(), 1_713_000_200);
    }

    #[test]
    fn persists_backup_manifest_entries_with_hashes_and_timestamps() {
        let mut store = SqliteStore::in_memory().unwrap();
        let manifest = BackupManifest {
            backup_id: "backup-1".to_string(),
            device_id: "device-1".to_string(),
            created_at: Timestamp::from_unix_seconds(1_713_000_300),
            profile: "default".to_string(),
            app_version: "0.1.0".to_string(),
            schema_version: SCHEMA_VERSION,
            source_root: "/mnt/device".to_string(),
            entries: vec![BackupManifestEntry {
                source_relative_path: ".adds/koreader/settings.reader.lua".to_string(),
                backup_relative_path: "payload/settings.reader.lua".to_string(),
                entry_kind: ManifestEntryKind::File,
                size_bytes: 4096,
                modified_at: Timestamp::from_unix_seconds(1_713_000_111),
                checksum: Some(checksum()),
                status: ManifestEntryStatus::Copied,
            }],
        };
        store.save_backup_manifest(&manifest).unwrap();

        let loaded = store.load_backup_manifest("backup-1").unwrap().unwrap();
        assert_eq!(loaded.created_at.unix_seconds(), 1_713_000_300);
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].size_bytes, 4096);
        assert_eq!(loaded.entries[0].modified_at.unix_seconds(), 1_713_000_111);
        assert_eq!(
            loaded.entries[0].checksum.as_ref().unwrap().hex(),
            checksum().hex()
        );
    }

    #[test]
    fn persists_release_metadata_cache() {
        let mut store = SqliteStore::in_memory().unwrap();
        let release = ReleaseMetadata {
            release_id: "release-1".to_string(),
            version: "v2026.04".to_string(),
            channel: ReleaseChannel::Stable,
            published_at: Timestamp::from_unix_seconds(1_713_000_000),
            fetched_at: Timestamp::from_unix_seconds(1_713_000_444),
            source_url: "https://example.invalid/releases/1".to_string(),
            assets: vec![ReleaseAsset {
                name: "koreader-kobo-v2026.04.zip".to_string(),
                download_url: "https://example.invalid/assets/1".to_string(),
                size_bytes: 12_345,
                content_type: Some("application/zip".to_string()),
                checksum: Some(checksum()),
            }],
        };

        store.put_release_metadata(&release).unwrap();
        let loaded = store.get_release_metadata("release-1").unwrap().unwrap();
        assert_eq!(loaded.version, "v2026.04");
        assert_eq!(loaded.fetched_at.unix_seconds(), 1_713_000_444);
        assert_eq!(loaded.assets[0].size_bytes, 12_345);
        assert_eq!(
            loaded.assets[0].checksum.as_ref().unwrap().hex(),
            checksum().hex()
        );
    }

    #[test]
    fn rejects_unknown_persisted_enum_values() {
        let store = SqliteStore::in_memory().unwrap();
        store
            .conn
            .execute(
                "INSERT INTO devices (id, kind, display_name, support_level, last_seen_at)
                 VALUES ('bad-device', 'kobo', 'Bad Device', 'mystery', 1)",
                [],
            )
            .unwrap();

        let error = store.get_device("bad-device").unwrap_err();
        assert!(matches!(
            error,
            PersistenceError::InvalidEnum {
                field: "support_level",
                ..
            }
        ));
    }

    #[test]
    fn rejects_unknown_persisted_target_kinds() {
        let store = SqliteStore::in_memory().unwrap();
        store
            .conn
            .execute(
                "INSERT INTO operation_logs (
                    plan_id, plan_item_id, execution_id, operation_id,
                    target_kind, target_value, severity, message, created_at
                 ) VALUES (1, 1, 1, 1, 'mystery', 'target', 'info', 'test', 1)",
                [],
            )
            .unwrap();

        let error = store.list_logs().unwrap_err();
        assert!(matches!(
            error,
            PersistenceError::InvalidEnum {
                field: "target_kind",
                ..
            }
        ));
    }

    #[test]
    fn enforces_foreign_keys() {
        let store = SqliteStore::in_memory().unwrap();
        let error = store
            .conn
            .execute(
                "INSERT INTO backup_manifest_entries (
                    backup_id, source_relative_path, backup_relative_path, entry_kind,
                    size_bytes, modified_at, checksum_algorithm, checksum_hex, status
                 ) VALUES ('missing', 'src', 'dst', 'file', 1, 1, NULL, NULL, 'copied')",
                [],
            )
            .unwrap_err();

        assert!(matches!(error, rusqlite::Error::SqliteFailure(_, _)));
    }

    #[test]
    fn rejects_non_absolute_backup_manifest_roots() {
        let mut store = SqliteStore::in_memory().unwrap();
        let manifest = BackupManifest {
            backup_id: "backup-relative".to_string(),
            device_id: "device-1".to_string(),
            created_at: Timestamp::from_unix_seconds(1_713_000_300),
            profile: "default".to_string(),
            app_version: "0.1.0".to_string(),
            schema_version: SCHEMA_VERSION,
            source_root: "relative/root".to_string(),
            entries: vec![],
        };

        let error = store.save_backup_manifest(&manifest).unwrap_err();
        assert!(matches!(
            error,
            PersistenceError::InvalidPath {
                field: "backup_manifest.source_root",
                ..
            }
        ));
    }

    #[test]
    fn rejects_lexically_escaping_backup_manifest_roots() {
        let mut store = SqliteStore::in_memory().unwrap();
        let manifest = BackupManifest {
            backup_id: "backup-escape".to_string(),
            device_id: "device-1".to_string(),
            created_at: Timestamp::from_unix_seconds(1_713_000_300),
            profile: "default".to_string(),
            app_version: "0.1.0".to_string(),
            schema_version: SCHEMA_VERSION,
            source_root: "/mnt/device/../escape".to_string(),
            entries: vec![],
        };

        let error = store.save_backup_manifest(&manifest).unwrap_err();
        assert!(matches!(
            error,
            PersistenceError::InvalidPath {
                field: "backup_manifest.source_root",
                ..
            }
        ));
    }
}
