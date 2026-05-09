use std::path::{Component, Path, PathBuf};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransportKind {
    LocalFilesystem,
    UsbMassStorage,
    NetworkShare,
    Adb,
    Ssh,
    MobileDocumentProvider,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Address {
    LocalPath(PathBuf),
    ScopedPath {
        transport: TransportKind,
        scope: String,
        relative_path: PathBuf,
    },
    Remote {
        transport: TransportKind,
        locator: String,
        path: String,
    },
    Logical {
        scheme: String,
        value: String,
    },
}

impl Address {
    pub fn filesystem(path: impl Into<PathBuf>) -> Self {
        Self::LocalPath(path.into())
    }

    pub fn scoped(
        transport: TransportKind,
        scope: impl Into<String>,
        relative_path: impl Into<PathBuf>,
    ) -> Result<Self, DomainError> {
        let scope = scope.into();
        if scope.trim().is_empty() {
            return Err(DomainError::Validation(
                "address scope must not be empty".to_string(),
            ));
        }

        Ok(Self::ScopedPath {
            transport,
            scope,
            relative_path: normalize_relative_path(relative_path.into())?,
        })
    }

    pub fn remote(
        transport: TransportKind,
        locator: impl Into<String>,
        path: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let locator = locator.into();
        let path = path.into();

        if locator.trim().is_empty() || path.trim().is_empty() {
            return Err(DomainError::Validation(
                "remote addresses require both locator and path".to_string(),
            ));
        }

        Ok(Self::Remote {
            transport,
            locator,
            path,
        })
    }

    pub fn logical(
        scheme: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let scheme = scheme.into();
        let value = value.into();

        if scheme.trim().is_empty() || value.trim().is_empty() {
            return Err(DomainError::Validation(
                "logical addresses require non-empty scheme and value".to_string(),
            ));
        }

        Ok(Self::Logical { scheme, value })
    }

    pub fn as_local_path(&self) -> Option<&Path> {
        match self {
            Self::LocalPath(path) => Some(path.as_path()),
            _ => None,
        }
    }
}

fn normalize_relative_path(path: PathBuf) -> Result<PathBuf, DomainError> {
    if path.is_absolute() {
        return Err(DomainError::Validation(format!(
            "address path must be relative: {}",
            path.display()
        )));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(DomainError::Validation(format!(
                    "address path must stay within its declared scope: {}",
                    path.display()
                )));
            }
        }
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_addresses_reject_absolute_or_traversing_paths() {
        assert!(matches!(
            Address::scoped(TransportKind::UsbMassStorage, "kobo", "/etc/passwd"),
            Err(DomainError::Validation(_))
        ));

        assert!(matches!(
            Address::scoped(TransportKind::Adb, "serial-1", "../escape"),
            Err(DomainError::Validation(_))
        ));
    }

    #[test]
    fn local_and_remote_addresses_preserve_transport_shape() {
        let local = Address::filesystem("/mnt/kobo");
        assert_eq!(local.as_local_path(), Some(Path::new("/mnt/kobo")));

        let remote = Address::remote(TransportKind::Ssh, "remarkable.local", "/home/root").unwrap();
        assert!(matches!(
            remote,
            Address::Remote {
                transport: TransportKind::Ssh,
                ..
            }
        ));
    }
}
