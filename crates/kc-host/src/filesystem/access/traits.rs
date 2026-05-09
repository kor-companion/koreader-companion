use std::path::{Path, PathBuf};

use kc_domain::{Address, DomainError, MetadataWriteRequest, ResourceMetadata};

pub trait HostFilesystem {
    fn canonicalize_dir(&self, path: &Path) -> Result<PathBuf, DomainError>;
    fn read_metadata(&self, address: &Address) -> Result<ResourceMetadata, DomainError>;
    fn write_metadata(
        &self,
        request: &MetadataWriteRequest,
    ) -> Result<ResourceMetadata, DomainError>;
}
