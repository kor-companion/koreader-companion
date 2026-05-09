use std::fs;
use std::path::{Path, PathBuf};

use super::{
    normalize::{normalize_path, normalize_root, reject_symlink_components},
    SafetyViolation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainedPath {
    pub root: PathBuf,
    pub full_path: PathBuf,
    pub relative_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainmentPolicy {
    root: PathBuf,
}

impl ContainmentPolicy {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, SafetyViolation> {
        let root = root.into();
        if !root.is_absolute() {
            return Err(SafetyViolation::RootMustBeAbsolute(root));
        }

        Ok(Self {
            root: normalize_root(&root)?,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn contain(&self, candidate: &Path) -> Result<ContainedPath, SafetyViolation> {
        match fs::symlink_metadata(&self.root) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(SafetyViolation::SymlinkComponent(self.root.clone()));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(SafetyViolation::PathResolution {
                    path: self.root.clone(),
                    message: error.to_string(),
                });
            }
        }

        let full_path = if candidate.is_absolute() {
            normalize_path(candidate)?
        } else {
            normalize_path(&self.root.join(candidate))?
        };

        if !full_path.starts_with(&self.root) {
            return Err(SafetyViolation::PathOutsideRoot {
                root: self.root.clone(),
                candidate: full_path,
            });
        }

        reject_symlink_components(&self.root, &full_path)?;

        let relative_path = full_path
            .strip_prefix(&self.root)
            .unwrap_or_else(|_| Path::new(""))
            .to_path_buf();

        Ok(ContainedPath {
            root: self.root.clone(),
            full_path,
            relative_path,
        })
    }
}
