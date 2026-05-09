use std::fs;
use std::path::{Component, Path, PathBuf};

use super::SafetyViolation;

pub(super) fn normalize_root(path: &Path) -> Result<PathBuf, SafetyViolation> {
    if path.exists() {
        fs::canonicalize(path).map_err(|error| SafetyViolation::PathResolution {
            path: path.to_path_buf(),
            message: error.to_string(),
        })
    } else {
        normalize_path(path)
    }
}

pub(super) fn reject_symlink_components(
    root: &Path,
    full_path: &Path,
) -> Result<(), SafetyViolation> {
    let relative = full_path
        .strip_prefix(root)
        .map_err(|_| SafetyViolation::PathOutsideRoot {
            root: root.to_path_buf(),
            candidate: full_path.to_path_buf(),
        })?;

    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());

        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(SafetyViolation::SymlinkComponent(current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => {
                return Err(SafetyViolation::PathResolution {
                    path: current,
                    message: error.to_string(),
                });
            }
        }
    }

    Ok(())
}

pub(super) fn normalize_path(path: &Path) -> Result<PathBuf, SafetyViolation> {
    let mut normalized = PathBuf::new();
    let mut floor = 0usize;

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => {
                normalized.push(prefix.as_os_str());
                floor = normalized.components().count();
            }
            Component::RootDir => {
                normalized.push(component.as_os_str());
                floor = normalized.components().count();
            }
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                if normalized.components().count() <= floor {
                    return Err(SafetyViolation::PathTraversal(path.to_path_buf()));
                }
                normalized.pop();
            }
        }
    }

    Ok(normalized)
}
