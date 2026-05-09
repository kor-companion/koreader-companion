use std::path::{Component, Path, PathBuf};

use kc_domain::{Address, ContainmentPolicy};

use crate::PersistenceError;

use super::projections::{derive_logical_suffix, derive_remote_suffix, normalize_relative_path};

pub fn derive_source_relative_path(
    source_root: &Address,
    source: &Address,
) -> Result<String, PersistenceError> {
    match (source_root, source) {
        (Address::LocalPath(root), Address::LocalPath(path)) => {
            validate_absolute_local_path("backup_manifest.source_root", root)?;
            validate_absolute_local_path("backup_manifest.entry.source", path)?;
            let contained = ContainmentPolicy::new(root)
                .map_err(|_| PersistenceError::InvalidPath {
                    field: "backup_manifest.source_root",
                    value: root.display().to_string(),
                })?
                .contain(path)
                .map_err(|_| PersistenceError::InvalidPath {
                    field: "backup_manifest.entry.source",
                    value: path.display().to_string(),
                })?;
            normalize_relative_path(&contained.relative_path, "backup_manifest.entry.source")
        }
        (
            Address::ScopedPath {
                transport: root_transport,
                scope: root_scope,
                relative_path: root_relative,
            },
            Address::ScopedPath {
                transport,
                scope,
                relative_path,
            },
        ) if transport == root_transport && scope == root_scope => relative_path
            .strip_prefix(root_relative)
            .map_err(|_| PersistenceError::InvalidPath {
                field: "backup_manifest.entry.source",
                value: format!("{source:?}"),
            })
            .and_then(|path| normalize_relative_path(path, "backup_manifest.entry.source")),
        (
            Address::Remote {
                transport: root_transport,
                locator: root_locator,
                path: root_path,
            },
            Address::Remote {
                transport,
                locator,
                path,
            },
        ) if transport == root_transport && locator == root_locator => {
            derive_remote_suffix(root_path, path, "backup_manifest.entry.source")
        }
        (
            Address::Logical {
                scheme: root_scheme,
                value: root_value,
            },
            Address::Logical { scheme, value },
        ) if scheme == root_scheme => {
            derive_logical_suffix(root_value, value, "backup_manifest.entry.source")
        }
        _ => Err(PersistenceError::InvalidPath {
            field: "backup_manifest.entry.source",
            value: format!("{source:?}"),
        }),
    }
}

pub fn validate_absolute_local_path(
    field: &'static str,
    value: &Path,
) -> Result<(), PersistenceError> {
    if !value.is_absolute() {
        return Err(PersistenceError::InvalidPath {
            field,
            value: value.display().to_string(),
        });
    }

    if value
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(PersistenceError::InvalidPath {
            field,
            value: value.display().to_string(),
        });
    }

    ContainmentPolicy::new(PathBuf::from(value))
        .map(|_| ())
        .map_err(PersistenceError::from)
}
