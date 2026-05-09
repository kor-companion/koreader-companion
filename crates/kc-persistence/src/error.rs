use std::error::Error;
use std::fmt;

use kc_domain::DomainError;
use kc_domain::SafetyViolation;

#[derive(Debug)]
pub enum PersistenceError {
    Sqlite(rusqlite::Error),
    Domain(DomainError),
    InvalidNumber(i64),
    InvalidEnum { field: &'static str, value: String },
    InvalidEncoding { field: &'static str, value: String },
    InvalidAddress { field: &'static str, value: String },
    InvalidTarget { field: &'static str, value: String },
    InvalidPath { field: &'static str, value: String },
    UnsupportedSchemaVersion { found: i64, supported: i64 },
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "sqlite error: {error}"),
            Self::Domain(error) => write!(f, "{error}"),
            Self::InvalidNumber(value) => write!(f, "invalid stored integer value: {value}"),
            Self::InvalidEnum { field, value } => write!(f, "invalid {field} value: {value}"),
            Self::InvalidEncoding { field, value } => {
                write!(f, "invalid encoded {field} value: {value}")
            }
            Self::InvalidAddress { field, value } => {
                write!(f, "invalid {field} address: {value}")
            }
            Self::InvalidTarget { field, value } => write!(f, "invalid {field} target: {value}"),
            Self::InvalidPath { field, value } => write!(f, "invalid {field} path: {value}"),
            Self::UnsupportedSchemaVersion { found, supported } => write!(
                f,
                "unsupported schema version {found}; this build supports up to {supported}"
            ),
        }
    }
}

impl Error for PersistenceError {}

impl From<rusqlite::Error> for PersistenceError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<DomainError> for PersistenceError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
    }
}

impl From<SafetyViolation> for PersistenceError {
    fn from(value: SafetyViolation) -> Self {
        Self::Domain(DomainError::Safety(value))
    }
}

impl From<PersistenceError> for DomainError {
    fn from(value: PersistenceError) -> Self {
        match value {
            PersistenceError::Domain(error) => error,
            error => DomainError::Validation(error.to_string()),
        }
    }
}
