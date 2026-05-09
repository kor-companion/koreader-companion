use kc_domain::{BackupManifestEntryRecord, BackupManifestRecord, ManifestRepository};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::codec::address::{encode_address, parse_address};
use crate::codec::enums::{encode_backup_entry_kind, parse_backup_entry_kind};
use crate::sqlite::store::{from_i64, to_i64, SqliteStore};
use crate::validation::{
    derive_source_relative_path, validate_address_projection, validate_source_root_address,
};
use crate::PersistenceError;

impl SqliteStore {
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
