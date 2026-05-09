use kc_domain::{
    Address, BackupEntryKind, BackupManifestEntryRecord, ManifestRepository, TransportKind,
};

use crate::sqlite::store::SqliteStore;
use crate::PersistenceError;

use super::fixtures::sample_manifest;

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
fn manifest_round_trips_scoped_source_roots() {
    let mut store = SqliteStore::in_memory().unwrap();
    let manifest = kc_domain::BackupManifestRecord {
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
    let manifest = kc_domain::BackupManifestRecord {
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
