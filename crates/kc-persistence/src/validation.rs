use std::path::{Component, Path, PathBuf};

use kc_domain::{Address, ContainmentPolicy};

use crate::PersistenceError;

pub fn validate_source_root_address(address: &Address) -> Result<(), PersistenceError> {
    match address {
        Address::LocalPath(path) => {
            validate_absolute_local_path("backup_manifest.source_root", path)
        }
        Address::ScopedPath { .. } | Address::Remote { .. } | Address::Logical { .. } => {
            validate_address_projection("backup_manifest.source_root", address).map(|_| ())
        }
    }
}

pub fn derive_source_relative_path(
    source_root: &Address,
    source: &Address,
) -> Result<String, PersistenceError> {
    match (source_root, source) {
        (Address::LocalPath(root), Address::LocalPath(path)) => {
            validate_absolute_local_path("backup_manifest.source_root", root)?;
            validate_absolute_local_path("backup_manifest.entry.source", path)?;
            let contained = ContainmentPolicy::new(root)
                .map_err(PersistenceError::from)?
                .contain(path)
                .map_err(PersistenceError::from)?;
            normalize_relative_path(&contained.relative_path)
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
            .and_then(normalize_relative_path),
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

pub fn validate_address_projection(
    field: &'static str,
    address: &Address,
) -> Result<String, PersistenceError> {
    match address {
        Address::LocalPath(path) => local_projection(path, field),
        Address::ScopedPath { relative_path, .. } => normalize_relative_path(relative_path),
        Address::Remote {
            locator: _, path, ..
        } => remote_projection(path, field),
        Address::Logical { value, .. } => normalize_slash_path(value, field, false),
    }
}

fn local_projection(path: &Path, field: &'static str) -> Result<String, PersistenceError> {
    validate_absolute_local_path(field, path)?;
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                return Err(PersistenceError::InvalidPath {
                    field,
                    value: path.display().to_string(),
                });
            }
        }
    }
    Ok(parts.join("/"))
}

fn remote_projection(path: &str, field: &'static str) -> Result<String, PersistenceError> {
    normalize_slash_path(path, field, true)
}

fn derive_remote_suffix(
    root: &str,
    path: &str,
    field: &'static str,
) -> Result<String, PersistenceError> {
    derive_slash_suffix(root, path, field, true)
}

fn derive_logical_suffix(
    root: &str,
    value: &str,
    field: &'static str,
) -> Result<String, PersistenceError> {
    derive_slash_suffix(root, value, field, false)
}

fn derive_slash_suffix(
    root: &str,
    candidate: &str,
    field: &'static str,
    require_absolute: bool,
) -> Result<String, PersistenceError> {
    let root_segments = normalize_slash_segments(root, field, require_absolute)?;
    let candidate_segments = normalize_slash_segments(candidate, field, require_absolute)?;
    if !candidate_segments.starts_with(&root_segments) {
        return Err(PersistenceError::InvalidPath {
            field,
            value: candidate.to_string(),
        });
    }
    Ok(candidate_segments[root_segments.len()..].join("/"))
}

fn normalize_relative_path(path: &Path) -> Result<String, PersistenceError> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(PersistenceError::InvalidPath {
                    field: "backup_manifest.entry_relative_path",
                    value: path.display().to_string(),
                });
            }
        }
    }
    Ok(parts.join("/"))
}

fn normalize_slash_path(
    value: &str,
    field: &'static str,
    require_absolute: bool,
) -> Result<String, PersistenceError> {
    Ok(normalize_slash_segments(value, field, require_absolute)?.join("/"))
}

fn normalize_slash_segments(
    value: &str,
    field: &'static str,
    require_absolute: bool,
) -> Result<Vec<String>, PersistenceError> {
    if require_absolute && !value.starts_with('/') {
        return Err(PersistenceError::InvalidPath {
            field,
            value: value.to_string(),
        });
    }

    let mut normalized = Vec::new();
    for segment in value.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            return Err(PersistenceError::InvalidPath {
                field,
                value: value.to_string(),
            });
        }
        normalized.push(segment.to_string());
    }
    Ok(normalized)
}

fn validate_absolute_local_path(field: &'static str, value: &Path) -> Result<(), PersistenceError> {
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
