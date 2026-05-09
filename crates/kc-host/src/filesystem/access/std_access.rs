use std::fs;
use std::path::{Path, PathBuf};

use kc_domain::{Address, DomainError, MetadataWriteRequest, ResourceMetadata};

use super::HostFilesystem;
use crate::filesystem::metadata::{classify_file_type, is_hidden, local_path_for};

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
