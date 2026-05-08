use std::error::Error;
use std::fmt;

use kc_domain::SafetyViolation;

use crate::ChecksumAlgorithm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadError {
    ArtifactNotFound {
        version: String,
        rule: String,
    },
    ArtifactAmbiguous {
        version: String,
        count: usize,
    },
    PrereleaseNotAllowed(String),
    InvalidChecksum {
        algorithm: ChecksumAlgorithm,
        value: String,
    },
    Safety(SafetyViolation),
}

impl fmt::Display for PayloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArtifactNotFound { version, rule } => {
                write!(
                    f,
                    "no release artifact matched {rule} for version {version}"
                )
            }
            Self::ArtifactAmbiguous { version, count } => {
                write!(f, "{count} release artifacts matched version {version}")
            }
            Self::PrereleaseNotAllowed(version) => {
                write!(
                    f,
                    "prerelease artifact selection is disabled for version {version}"
                )
            }
            Self::InvalidChecksum { algorithm, value } => {
                write!(f, "invalid {} checksum: {value}", algorithm.as_str())
            }
            Self::Safety(error) => write!(f, "{error}"),
        }
    }
}

impl Error for PayloadError {}

impl From<SafetyViolation> for PayloadError {
    fn from(value: SafetyViolation) -> Self {
        Self::Safety(value)
    }
}
