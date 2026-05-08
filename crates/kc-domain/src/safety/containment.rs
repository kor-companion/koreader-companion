use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyViolation {
    RootMustBeAbsolute(PathBuf),
    PathTraversal(PathBuf),
    PathOutsideRoot { root: PathBuf, candidate: PathBuf },
    SymlinkComponent(PathBuf),
    PathResolution { path: PathBuf, message: String },
}

impl fmt::Display for SafetyViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootMustBeAbsolute(path) => {
                write!(f, "containment root must be absolute: {}", path.display())
            }
            Self::PathTraversal(path) => {
                write!(f, "path escapes containment root: {}", path.display())
            }
            Self::PathOutsideRoot { root, candidate } => write!(
                f,
                "path {} is outside containment root {}",
                candidate.display(),
                root.display()
            ),
            Self::SymlinkComponent(path) => {
                write!(
                    f,
                    "path component resolves through a symlink: {}",
                    path.display()
                )
            }
            Self::PathResolution { path, message } => {
                write!(f, "failed to resolve path {}: {message}", path.display())
            }
        }
    }
}

impl Error for SafetyViolation {}

fn normalize_root(path: &Path) -> Result<PathBuf, SafetyViolation> {
    if path.exists() {
        fs::canonicalize(path).map_err(|error| SafetyViolation::PathResolution {
            path: path.to_path_buf(),
            message: error.to_string(),
        })
    } else {
        normalize_path(path)
    }
}

fn reject_symlink_components(root: &Path, full_path: &Path) -> Result<(), SafetyViolation> {
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

fn normalize_path(path: &Path) -> Result<PathBuf, SafetyViolation> {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{ContainmentPolicy, SafetyViolation};

    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    #[test]
    fn containment_policy_rejects_escape_paths() {
        let policy = ContainmentPolicy::new("/mnt/kobo").unwrap();

        let contained = policy
            .contain(Path::new(".adds/../.adds/koreader"))
            .unwrap();
        assert_eq!(
            contained.full_path,
            PathBuf::from("/mnt/kobo/.adds/koreader")
        );
        assert_eq!(contained.relative_path, PathBuf::from(".adds/koreader"));

        let error = policy.contain(Path::new("../etc/passwd")).unwrap_err();
        assert_eq!(
            error,
            SafetyViolation::PathOutsideRoot {
                root: PathBuf::from("/mnt/kobo"),
                candidate: PathBuf::from("/mnt/etc/passwd"),
            }
        );
    }

    #[cfg(unix)]
    #[test]
    fn containment_policy_rejects_symlink_components() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("kc-domain-{unique}"));
        let outside = std::env::temp_dir().join(format!("kc-domain-outside-{unique}"));

        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        symlink(&outside, root.join("escape")).unwrap();

        let policy = ContainmentPolicy::new(&root).unwrap();
        let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
        assert_eq!(
            error,
            SafetyViolation::SymlinkComponent(root.join("escape"))
        );

        fs::remove_file(root.join("escape")).unwrap();
        fs::remove_dir_all(&root).unwrap();
        fs::remove_dir_all(&outside).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn containment_policy_rejects_root_symlink_created_after_init() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("kc-domain-late-root-{unique}"));
        let outside = std::env::temp_dir().join(format!("kc-domain-late-outside-{unique}"));

        let policy = ContainmentPolicy::new(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        symlink(&outside, &root).unwrap();

        let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
        assert_eq!(error, SafetyViolation::SymlinkComponent(root.clone()));

        fs::remove_file(&root).unwrap();
        fs::remove_dir_all(&outside).unwrap();
    }
}
