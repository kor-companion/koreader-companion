use std::fs;
use std::path::Path;

use kc_domain::{Address, DomainError, ResourceKind};

pub fn local_path_for(address: &Address) -> Result<&Path, DomainError> {
    address.as_local_path().ok_or_else(|| {
        DomainError::Unsupported(
            "this host adapter only supports local filesystem metadata operations".to_string(),
        )
    })
}

pub fn classify_file_type(metadata: &fs::Metadata) -> ResourceKind {
    let file_type = metadata.file_type();
    if file_type.is_file() {
        ResourceKind::File
    } else if file_type.is_dir() {
        ResourceKind::Directory
    } else if file_type.is_symlink() {
        ResourceKind::Symlink
    } else {
        ResourceKind::Other
    }
}

pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}
