use crate::Checksum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationState {
    Pending,
    Verified,
    Warning,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChecksumValidation {
    Missing,
    Verified(Checksum),
    Mismatch {
        expected: Checksum,
        actual_hex: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutValidation {
    Unknown,
    Valid,
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub state: ValidationState,
    pub checksum: ChecksumValidation,
    pub layout: LayoutValidation,
}

impl ValidationResult {
    pub fn verified(checksum: Checksum) -> Self {
        Self {
            state: ValidationState::Verified,
            checksum: ChecksumValidation::Verified(checksum),
            layout: LayoutValidation::Valid,
        }
    }

    pub fn warning(checksum: ChecksumValidation, layout: LayoutValidation) -> Self {
        Self {
            state: ValidationState::Warning,
            checksum,
            layout,
        }
    }

    pub fn rejected(reason: impl Into<String>) -> Self {
        Self {
            state: ValidationState::Rejected,
            checksum: ChecksumValidation::Missing,
            layout: LayoutValidation::Invalid(reason.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Checksum, ChecksumAlgorithm};

    use super::{ChecksumValidation, LayoutValidation, ValidationResult, ValidationState};

    #[test]
    fn validation_types_preserve_payload_review_state() {
        let checksum = Checksum::new(
            ChecksumAlgorithm::Sha256,
            "abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        )
        .unwrap();

        assert_eq!(
            ValidationResult::verified(checksum.clone()).state,
            ValidationState::Verified
        );
        assert_eq!(
            ValidationResult::warning(
                ChecksumValidation::Mismatch {
                    expected: checksum,
                    actual_hex: "deadbeef".to_string(),
                },
                LayoutValidation::Unknown,
            )
            .state,
            ValidationState::Warning
        );
        assert_eq!(
            ValidationResult::rejected("missing launcher").state,
            ValidationState::Rejected
        );
    }
}
