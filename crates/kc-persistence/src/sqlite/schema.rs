use rusqlite::Connection;

use crate::PersistenceError;

pub const SCHEMA_VERSION: i64 = 1;

pub fn enable_foreign_keys(conn: &Connection) -> Result<(), PersistenceError> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(())
}

pub fn bootstrap(conn: &Connection) -> Result<(), PersistenceError> {
    conn.execute_batch(
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
             recorded_at_unix INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS devices (
             id TEXT PRIMARY KEY,
             kind TEXT NOT NULL,
             display_name TEXT NOT NULL,
             support_level TEXT NOT NULL,
             last_seen_at_unix INTEGER NOT NULL,
             last_host_id TEXT,
             last_address TEXT
         );
         CREATE TABLE IF NOT EXISTS backup_manifests (
             manifest_id TEXT PRIMARY KEY,
             device_id TEXT NOT NULL,
             created_at_unix INTEGER NOT NULL,
             profile TEXT NOT NULL,
             app_version TEXT NOT NULL,
             schema_version INTEGER NOT NULL,
             source_root TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS backup_manifest_entries (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             manifest_id TEXT NOT NULL,
             source_address TEXT NOT NULL,
             source_relative_path TEXT NOT NULL,
             backup_address TEXT NOT NULL,
             backup_relative_path TEXT NOT NULL,
             entry_kind TEXT NOT NULL,
             size_bytes INTEGER NOT NULL,
             checksum_hex TEXT,
             FOREIGN KEY(manifest_id) REFERENCES backup_manifests(manifest_id) ON DELETE CASCADE
         );
         CREATE TABLE IF NOT EXISTS release_metadata_cache (
             cache_key TEXT PRIMARY KEY,
             release_id TEXT NOT NULL,
             version TEXT NOT NULL,
             channel TEXT NOT NULL,
             published_at_unix INTEGER NOT NULL,
             fetched_at_unix INTEGER NOT NULL,
             source_url TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS release_artifacts (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             cache_key TEXT NOT NULL,
             name TEXT NOT NULL,
             download_url TEXT NOT NULL,
             size_bytes INTEGER NOT NULL,
             content_type TEXT,
              checksum_hex TEXT,
              FOREIGN KEY(cache_key) REFERENCES release_metadata_cache(cache_key) ON DELETE CASCADE
          );
          INSERT OR IGNORE INTO schema_migrations (version, applied_at)
          VALUES (1, strftime('%s','now'));
          COMMIT;",
    )?;
    Ok(())
}
