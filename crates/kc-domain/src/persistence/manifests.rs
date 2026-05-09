use crate::{Address, DomainError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupEntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifestEntryRecord {
    pub source: Address,
    pub backup: Address,
    pub kind: BackupEntryKind,
    pub size_bytes: u64,
    pub checksum_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupManifestRecord {
    pub manifest_id: String,
    pub device_id: String,
    pub created_at_unix: i64,
    pub profile: String,
    pub app_version: String,
    pub schema_version: i64,
    pub source_root: Address,
    pub entries: Vec<BackupManifestEntryRecord>,
}

pub trait ManifestRepository {
    fn save_manifest(&mut self, manifest: &BackupManifestRecord) -> Result<(), DomainError>;
    fn load_manifest(&self, id: &str) -> Result<Option<BackupManifestRecord>, DomainError>;
}
