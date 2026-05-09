use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use kc_domain::LogSeverity;
use rusqlite::Connection;

use crate::sqlite::schema::{bootstrap, enable_foreign_keys, SCHEMA_VERSION};
use crate::PersistenceError;

mod devices;
mod events;
mod logs;
mod manifests;
mod releases;

pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, PersistenceError> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        enable_foreign_keys(&store.conn)?;
        bootstrap(&store.conn)?;
        store.ensure_supported_schema()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self, PersistenceError> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        enable_foreign_keys(&store.conn)?;
        bootstrap(&store.conn)?;
        store.ensure_supported_schema()?;
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

    fn ensure_supported_schema(&self) -> Result<(), PersistenceError> {
        let version = self.schema_version()?;
        if version > SCHEMA_VERSION {
            return Err(PersistenceError::UnsupportedSchemaVersion {
                found: version,
                supported: SCHEMA_VERSION,
            });
        }

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn connection(&self) -> &Connection {
        &self.conn
    }
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
