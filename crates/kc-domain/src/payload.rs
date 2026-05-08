use crate::{Address, DomainError, WorkflowKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    ReleaseArtifact,
    BackupArchive,
    LocalDirectory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadRequest {
    pub workflow: WorkflowKind,
    pub identifier: String,
    pub preferred_source: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadSelection {
    pub kind: PayloadKind,
    pub identifier: String,
    pub display_name: String,
    pub address: Option<Address>,
}

pub trait PayloadSource {
    fn kind(&self) -> PayloadKind;
    fn resolve(&self, request: &PayloadRequest) -> Result<PayloadSelection, DomainError>;
}
