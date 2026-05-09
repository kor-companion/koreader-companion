use std::error::Error;
use std::fmt;
use std::path::PathBuf;

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
