use std::path::{Path, PathBuf};

use kc_domain::{
    Address, DomainError, HostAccess, HostKind, HostOperationTarget, MetadataWriteRequest,
    MountPoint, ResourceKind,
};

use super::{host::FilesystemHost, InMemoryHostFilesystem};
use crate::HostAdapterDescriptor;

#[test]
fn filesystem_host_validates_manual_paths_through_boundary() {
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::new([PathBuf::from("/mnt/kobo")]),
        vec![],
    );

    let validated = host.validate_manual_path(Path::new("/mnt/kobo")).unwrap();
    assert_eq!(validated.path, PathBuf::from("/mnt/kobo"));

    let error = host
        .validate_manual_path(Path::new("/mnt/missing"))
        .unwrap_err();
    assert!(matches!(error, DomainError::Validation(_)));
}

#[test]
fn host_metadata_boundary_exposes_symlink_aware_shapes() {
    let config_path = PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf");
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::with_entries(
            [
                PathBuf::from("/mnt/kobo"),
                PathBuf::from("/mnt/kobo/.kobo/Kobo"),
            ],
            [config_path.clone()],
        )
        .with_read_only_paths([config_path.clone()]),
        vec![],
    );

    let metadata = host
        .read_metadata(&Address::filesystem(config_path.clone()))
        .unwrap();
    assert_eq!(metadata.kind, Some(ResourceKind::File));
    assert_eq!(metadata.read_only, Some(true));

    let updated = host
        .write_metadata(&MetadataWriteRequest {
            address: Address::filesystem(config_path.clone()),
            read_only: Some(false),
            hidden: Some(false),
        })
        .unwrap();
    assert_eq!(updated.read_only, Some(false));
}

#[test]
fn host_operation_readiness_stays_honest_about_unimplemented_sync_and_eject() {
    let mount = MountPoint {
        id: "kobo".to_string(),
        root: PathBuf::from("/mnt/kobo"),
        name: Some("KOBOeReader".to_string()),
        removable: true,
    };
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::new([mount.root.clone()]),
        vec![mount.clone()],
    );

    let sync = host
        .sync_readiness(&HostOperationTarget::Mount(mount.clone()))
        .unwrap();
    assert!(!sync.ready);
    assert!(sync.blockers[0].contains("not implemented"));
    assert!(sync.guidance[0].contains("manual"));

    let eject = host
        .eject_readiness(&HostOperationTarget::Mount(mount))
        .unwrap();
    assert!(!eject.ready);
    assert!(eject.blockers[0].contains("not implemented"));
    assert!(eject.guidance[0].contains("desktop"));
}

#[test]
fn non_removable_mounts_are_not_presented_as_ejectable() {
    let mount = MountPoint {
        id: "system".to_string(),
        root: PathBuf::from("/mnt/system"),
        name: Some("System".to_string()),
        removable: false,
    };
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::macos(),
        InMemoryHostFilesystem::new([mount.root.clone()]),
        vec![mount.clone()],
    );

    let readiness = host
        .eject_readiness(&HostOperationTarget::Mount(mount))
        .unwrap();
    assert!(!readiness.ready);
    assert!(readiness.blockers[0].contains("not marked removable"));
}

#[test]
fn current_adapter_tracks_runtime_host_kind() {
    let adapter = crate::current_host_adapter();
    assert!(matches!(
        adapter.descriptor.kind,
        HostKind::Linux | HostKind::MacOs | HostKind::Windows | HostKind::Other(_)
    ));
}

#[test]
fn in_memory_metadata_matches_missing_path_semantics() {
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::default(),
        vec![],
    );

    let metadata = host
        .read_metadata(&Address::filesystem("/mnt/missing/file.txt"))
        .unwrap();
    assert!(!metadata.exists);
    assert_eq!(metadata.kind, None);
    assert_eq!(metadata.size_bytes, None);
    assert_eq!(metadata.read_only, None);
}
