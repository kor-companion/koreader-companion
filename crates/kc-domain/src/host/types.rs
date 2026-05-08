use std::path::PathBuf;

use crate::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostKind {
    Linux,
    MacOs,
    Windows,
    Android,
    Ios,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostDescriptor {
    pub id: String,
    pub kind: HostKind,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountPoint {
    pub id: String,
    pub root: PathBuf,
    pub name: Option<String>,
    pub removable: bool,
}

impl MountPoint {
    pub fn root_address(&self) -> Address {
        Address::filesystem(self.root.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPath {
    pub path: PathBuf,
}

impl ValidatedPath {
    pub fn address(&self) -> Address {
        Address::filesystem(self.path.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedAddress {
    pub address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceMetadata {
    pub address: Address,
    pub exists: bool,
    pub kind: Option<ResourceKind>,
    pub size_bytes: Option<u64>,
    pub read_only: Option<bool>,
    pub hidden: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataWriteRequest {
    pub address: Address,
    pub read_only: Option<bool>,
    pub hidden: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostOperationTarget {
    Mount(MountPoint),
    Address(Address),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostOperationReadiness {
    pub ready: bool,
    pub blockers: Vec<String>,
    pub guidance: Vec<String>,
}

impl HostOperationReadiness {
    pub fn ready() -> Self {
        Self {
            ready: true,
            blockers: Vec::new(),
            guidance: Vec::new(),
        }
    }

    pub fn blocked(blockers: Vec<String>, guidance: impl IntoIterator<Item = String>) -> Self {
        Self {
            ready: false,
            blockers,
            guidance: guidance.into_iter().collect(),
        }
    }

    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::blocked(vec![message.into()], Vec::<String>::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSyncResult {
    pub target: HostOperationTarget,
    pub requires_manual_completion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostEjectResult {
    pub target: HostOperationTarget,
    pub requires_manual_completion: bool,
}
