use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kc_domain::{Address, DomainError, MetadataWriteRequest, ResourceKind, ResourceMetadata};

use super::HostFilesystem;
use crate::filesystem::metadata::{is_hidden, local_path_for};

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
