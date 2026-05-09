use kc_domain::Address;

use crate::ValidationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageSource {
    Download,
    LocalArtifact,
    Cache,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageResult {
    pub source: StageSource,
    pub staged_at: Address,
    pub validation: ValidationResult,
}

#[cfg(test)]
mod tests {
    use kc_domain::Address;

    use crate::{Checksum, ChecksumAlgorithm, StageResult, StageSource, ValidationResult};

    #[test]
    fn stage_results_are_address_based_and_frontend_neutral() {
        let result = StageResult {
            source: StageSource::Cache,
            staged_at: Address::filesystem("/tmp/koreader.zip"),
            validation: ValidationResult::verified(
                Checksum::new(
                    ChecksumAlgorithm::Sha256,
                    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                )
                .unwrap(),
            ),
        };

        assert_eq!(result.staged_at, Address::filesystem("/tmp/koreader.zip"));
    }
}
