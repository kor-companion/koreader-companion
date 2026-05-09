use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CachedReleaseChannel {
    Stable,
    Prerelease,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactRecord {
    pub name: String,
    pub download_url: String,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub checksum_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedReleaseMetadata {
    pub cache_key: String,
    pub release_id: String,
    pub version: String,
    pub channel: CachedReleaseChannel,
    pub published_at_unix: i64,
    pub fetched_at_unix: i64,
    pub source_url: String,
    pub artifacts: Vec<ReleaseArtifactRecord>,
}

pub trait ReleaseMetadataCache {
    fn put_release_metadata(&mut self, release: &CachedReleaseMetadata) -> Result<(), DomainError>;
    fn get_release_metadata(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, DomainError>;
}
