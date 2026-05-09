use std::path::PathBuf;

use kc_domain::{Address, DomainError, MetadataWriteRequest, ResourceKind};

use super::{HostFilesystem, InMemoryHostFilesystem};

#[test]
fn in_memory_metadata_reads_and_writes_are_boundary_safe() {
    let path = PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf");
    let filesystem = InMemoryHostFilesystem::with_entries(
        [PathBuf::from("/mnt/kobo/.kobo/Kobo")],
        [path.clone()],
    )
    .with_read_only_paths([path.clone()]);

    let metadata = filesystem
        .read_metadata(&Address::filesystem(path.clone()))
        .unwrap();
    assert_eq!(metadata.kind, Some(ResourceKind::File));
    assert_eq!(metadata.read_only, Some(true));
    assert_eq!(metadata.hidden, Some(false));

    let updated = filesystem
        .write_metadata(&MetadataWriteRequest {
            address: Address::filesystem(path.clone()),
            read_only: Some(false),
            hidden: Some(false),
        })
        .unwrap();
    assert_eq!(updated.read_only, Some(false));
}

#[test]
fn non_local_metadata_requests_are_rejected() {
    let filesystem = InMemoryHostFilesystem::default();
    let address = Address::remote(kc_domain::TransportKind::Ssh, "host", "/tmp").unwrap();
    assert!(matches!(
        filesystem.read_metadata(&address),
        Err(DomainError::Unsupported(_))
    ));
}
