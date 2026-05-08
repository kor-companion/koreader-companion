use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use kc_domain::{Address, DomainError, MetadataWriteRequest, ResourceKind, ResourceMetadata};

use super::metadata::{classify_file_type, is_hidden, local_path_for};

pub trait HostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError>;
    fn read_metadata(&self, address: &Address) -> Result<ResourceMetadata, DomainError>;
    fn write_metadata(
        &self,
        request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StdHostFilesystem;

impl HostFilesystem for StdHostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError> {
        let canonical = fs::canonicalize(path).map_err(|error| {
            DomainError::Validation(format!(
                "failed to access manual path {}: {error}",
                path.display()
            ))
        })?;

        if !canonical.is_dir() {
            return Err(DomainError::Validation(format!(
                "manual path {} is not a directory",
                canonical.display()
            )));
        }

        Ok(canonical)
    }

    fn read_metadata(&self, address: &Address) -> Result<ResourceMetadata, DomainError> {
        let path = local_path_for(address)?;

        match fs::symlink_metadata(path) {
            Ok(metadata) => Ok(ResourceMetadata {
                address: address.clone(),
                exists: true,
                kind: Some(classify_file_type(&metadata)),
                size_bytes: Some(metadata.len()),
                read_only: Some(metadata.permissions().readonly()),
                hidden: Some(is_hidden(path)),
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ResourceMetadata {
                address: address.clone(),
                exists: false,
                kind: None,
                size_bytes: None,
                read_only: None,
                hidden: Some(is_hidden(path)),
            }),
            Err(error) => Err(DomainError::Validation(format!(
                "failed to read metadata for {}: {error}",
                path.display()
            ))),
        }
    }

    fn write_metadata(
        &self,
        request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError> {
        let path = local_path_for(&request.address)?;
        let metadata = fs::symlink_metadata(path).map_err(|error| {
            DomainError::Validation(format!(
                "failed to update metadata for {}: {error}",
                path.display()
            ))
        })?;

        if metadata.file_type().is_symlink() {
            return Err(DomainError::Unsupported(format!(
                "host metadata writes do not operate on symlinks: {}",
                path.display()
            )));
        }

        let current_hidden = is_hidden(path);
        if let Some(hidden) = request.hidden {
            if hidden != current_hidden {
                return Err(DomainError::Unsupported(
                    "changing hidden metadata is not implemented by this host adapter".to_string(),
                ));
            }
        }

        if let Some(read_only) = request.read_only {
            let mut permissions = metadata.permissions();
            permissions.set_readonly(read_only);
            fs::set_permissions(path, permissions).map_err(|error| {
                DomainError::Validation(format!(
                    "failed to update permissions for {}: {error}",
                    path.display()
                ))
            })?;
        }

        self.read_metadata(&request.address)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryHostFilesystem {
    directories: BTreeSet<PathBuf>,
    files: BTreeSet<PathBuf>,
    symlinks: BTreeSet<PathBuf>,
    read_only: RefCell<BTreeSet<PathBuf>>,
}

impl InMemoryHostFilesystem {
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            directories: paths.into_iter().collect(),
            files: BTreeSet::new(),
            symlinks: BTreeSet::new(),
            read_only: RefCell::new(BTreeSet::new()),
        }
    }

    pub fn with_entries(
        directories: impl IntoIterator<Item = PathBuf>,
        files: impl IntoIterator<Item = PathBuf>,
    ) -> Self {
        Self {
            directories: directories.into_iter().collect(),
            files: files.into_iter().collect(),
            symlinks: BTreeSet::new(),
            read_only: RefCell::new(BTreeSet::new()),
        }
    }

    pub fn with_symlinks(mut self, paths: impl IntoIterator<Item = PathBuf>) -> Self {
        self.symlinks = paths.into_iter().collect();
        self
    }

    pub fn with_read_only_paths(self, paths: impl IntoIterator<Item = PathBuf>) -> Self {
        *self.read_only.borrow_mut() = paths.into_iter().collect();
        self
    }

    fn metadata_for_path(&self, path: &Path) -> ResourceMetadata {
        let kind = if self.symlinks.contains(path) {
            Some(ResourceKind::Symlink)
        } else if self.directories.contains(path) {
            Some(ResourceKind::Directory)
        } else if self.files.contains(path) {
            Some(ResourceKind::File)
        } else {
            None
        };

        ResourceMetadata {
            address: Address::filesystem(path),
            exists: kind.is_some(),
            kind,
            size_bytes: if self.files.contains(path) {
                Some(1)
            } else if kind.is_some() {
                Some(0)
            } else {
                None
            },
            read_only: kind.map(|_| self.read_only.borrow().contains(path)),
            hidden: Some(is_hidden(path)),
        }
    }
}

impl HostFilesystem for InMemoryHostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError> {
        let candidate = path.to_path_buf();
        if self.directories.contains(&candidate) {
            Ok(candidate)
        } else {
            Err(DomainError::Validation(format!(
                "manual path {} is not available in the host fixture",
                path.display()
            )))
        }
    }

    fn read_metadata(&self, address: &Address) -> Result<ResourceMetadata, DomainError> {
        let path = local_path_for(address)?;
        Ok(self.metadata_for_path(path))
    }

    fn write_metadata(
        &self,
        request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError> {
        let path = local_path_for(&request.address)?;
        let metadata = self.metadata_for_path(path);

        if !metadata.exists {
            return Err(DomainError::Validation(format!(
                "host fixture does not contain {}",
                path.display()
            )));
        }

        if metadata.kind == Some(ResourceKind::Symlink) {
            return Err(DomainError::Unsupported(format!(
                "host metadata writes do not operate on symlinks: {}",
                path.display()
            )));
        }

        if let Some(hidden) = request.hidden {
            if metadata.hidden != Some(hidden) {
                return Err(DomainError::Unsupported(
                    "changing hidden metadata is not implemented by this host fixture".to_string(),
                ));
            }
        }

        if let Some(read_only) = request.read_only {
            let mut read_only_paths = self.read_only.borrow_mut();
            if read_only {
                read_only_paths.insert(path.to_path_buf());
            } else {
                read_only_paths.remove(path);
            }
        }

        self.read_metadata(&request.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
