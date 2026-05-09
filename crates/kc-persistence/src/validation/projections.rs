use std::path::{Component, Path};

use kc_domain::Address;

use crate::PersistenceError;

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
    super::validate_absolute_local_path(field, path)?;
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

pub(super) fn derive_remote_suffix(
    root: &str,
    path: &str,
    field: &'static str,
) -> Result<String, PersistenceError> {
    derive_slash_suffix(root, path, field, true)
}

pub(super) fn derive_logical_suffix(
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

pub(super) fn normalize_relative_path(path: &Path) -> Result<String, PersistenceError> {
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
