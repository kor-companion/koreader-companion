use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use kc_domain::{ContainedPath, ContainmentPolicy, SafetyViolation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(i64);

impl Timestamp {
    pub const fn from_unix_seconds(value: i64) -> Self {
        Self(value)
    }

    pub const fn unix_seconds(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseChannel {
    Stable,
    Prerelease,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub download_url: String,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub checksum: Option<Checksum>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseMetadata {
    pub release_id: String,
    pub version: String,
    pub channel: ReleaseChannel,
    pub published_at: Timestamp,
    pub fetched_at: Timestamp,
    pub source_url: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRule {
    pub name_contains: String,
    pub extension: String,
    pub allow_prerelease: bool,
}

impl ArtifactRule {
    pub fn matches(&self, asset: &ReleaseAsset) -> bool {
        asset.name.contains(&self.name_contains) && asset.name.ends_with(&self.extension)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSelection {
    pub release_id: String,
    pub version: String,
    pub asset: ReleaseAsset,
}

pub fn select_artifact(
    release: &ReleaseMetadata,
    rule: &ArtifactRule,
) -> Result<ArtifactSelection, PayloadError> {
    if release.channel == ReleaseChannel::Prerelease && !rule.allow_prerelease {
        return Err(PayloadError::PrereleaseNotAllowed(release.version.clone()));
    }

    let matches = release
        .assets
        .iter()
        .filter(|asset| rule.matches(asset))
        .cloned()
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [asset] => Ok(ArtifactSelection {
            release_id: release.release_id.clone(),
            version: release.version.clone(),
            asset: asset.clone(),
        }),
        [] => Err(PayloadError::ArtifactNotFound {
            version: release.version.clone(),
            rule: format!(
                "contains '{}' and ends with '{}'",
                rule.name_contains, rule.extension
            ),
        }),
        _ => Err(PayloadError::ArtifactAmbiguous {
            version: release.version.clone(),
            count: matches.len(),
        }),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Sha256,
    Sha512,
}

impl ChecksumAlgorithm {
    pub const fn expected_hex_len(self) -> usize {
        match self {
            Self::Sha256 => 64,
            Self::Sha512 => 128,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checksum {
    algorithm: ChecksumAlgorithm,
    hex: String,
}

impl Checksum {
    pub fn new(algorithm: ChecksumAlgorithm, hex: impl Into<String>) -> Result<Self, PayloadError> {
        let hex = hex.into().to_ascii_lowercase();
        if hex.len() != algorithm.expected_hex_len() || !hex.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(PayloadError::InvalidChecksum {
                algorithm,
                value: hex,
            });
        }

        Ok(Self { algorithm, hex })
    }

    pub const fn algorithm(&self) -> ChecksumAlgorithm {
        self.algorithm
    }

    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn matches(&self, actual_hex: &str) -> bool {
        self.hex == actual_hex.to_ascii_lowercase()
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageSource {
    Download,
    LocalArtifact,
    Cache,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageResult {
    pub source: StageSource,
    pub staged_path: PathBuf,
    pub validation: ValidationResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionTarget {
    pub relative_path: PathBuf,
    pub destination: ContainedPath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionPlan {
    pub archive_path: PathBuf,
    pub boundary_root: PathBuf,
    pub targets: Vec<ExtractionTarget>,
}

impl ExtractionPlan {
    pub fn new(
        archive_path: impl Into<PathBuf>,
        boundary: &ContainmentPolicy,
        relative_paths: impl IntoIterator<Item = PathBuf>,
    ) -> Result<Self, PayloadError> {
        let archive_path = archive_path.into();
        let mut targets = Vec::new();

        for relative_path in relative_paths {
            let destination = boundary.contain(&relative_path)?;
            targets.push(ExtractionTarget {
                relative_path,
                destination,
            });
        }

        Ok(Self {
            archive_path,
            boundary_root: boundary.root().to_path_buf(),
            targets,
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_release() -> ReleaseMetadata {
        ReleaseMetadata {
            release_id: "release-42".to_string(),
            version: "v2026.04".to_string(),
            channel: ReleaseChannel::Stable,
            published_at: Timestamp::from_unix_seconds(1_713_000_000),
            fetched_at: Timestamp::from_unix_seconds(1_713_000_060),
            source_url: "https://example.invalid/releases/42".to_string(),
            assets: vec![
                ReleaseAsset {
                    name: "koreader-kobo-v2026.04.zip".to_string(),
                    download_url: "https://example.invalid/assets/kobo.zip".to_string(),
                    size_bytes: 10,
                    content_type: Some("application/zip".to_string()),
                    checksum: Some(
                        Checksum::new(
                            ChecksumAlgorithm::Sha256,
                            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                        )
                        .unwrap(),
                    ),
                },
                ReleaseAsset {
                    name: "koreader-remarkable-v2026.04.zip".to_string(),
                    download_url: "https://example.invalid/assets/rm.zip".to_string(),
                    size_bytes: 11,
                    content_type: Some("application/zip".to_string()),
                    checksum: None,
                },
            ],
        }
    }

    #[test]
    fn selects_unique_release_artifact() {
        let selection = select_artifact(
            &sample_release(),
            &ArtifactRule {
                name_contains: "kobo".to_string(),
                extension: ".zip".to_string(),
                allow_prerelease: false,
            },
        )
        .unwrap();

        assert_eq!(selection.version, "v2026.04");
        assert_eq!(selection.asset.name, "koreader-kobo-v2026.04.zip");
    }

    #[test]
    fn rejects_ambiguous_or_missing_artifacts() {
        let release = ReleaseMetadata {
            assets: vec![
                ReleaseAsset {
                    name: "koreader-kobo-a.zip".to_string(),
                    download_url: "a".to_string(),
                    size_bytes: 1,
                    content_type: None,
                    checksum: None,
                },
                ReleaseAsset {
                    name: "koreader-kobo-b.zip".to_string(),
                    download_url: "b".to_string(),
                    size_bytes: 1,
                    content_type: None,
                    checksum: None,
                },
            ],
            ..sample_release()
        };

        let rule = ArtifactRule {
            name_contains: "kobo".to_string(),
            extension: ".zip".to_string(),
            allow_prerelease: false,
        };

        assert!(matches!(
            select_artifact(&release, &rule),
            Err(PayloadError::ArtifactAmbiguous { count: 2, .. })
        ));

        assert!(matches!(
            select_artifact(
                &sample_release(),
                &ArtifactRule {
                    name_contains: "android".to_string(),
                    ..rule
                }
            ),
            Err(PayloadError::ArtifactNotFound { .. })
        ));
    }

    #[test]
    fn checksum_and_validation_types_preserve_state() {
        let checksum = Checksum::new(
            ChecksumAlgorithm::Sha256,
            "abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        )
        .unwrap();
        assert!(
            checksum.matches("ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCD")
        );

        let verified = ValidationResult::verified(checksum.clone());
        assert_eq!(verified.state, ValidationState::Verified);

        let warning = ValidationResult::warning(
            ChecksumValidation::Mismatch {
                expected: checksum.clone(),
                actual_hex: "deadbeef".to_string(),
            },
            LayoutValidation::Unknown,
        );
        assert_eq!(warning.state, ValidationState::Warning);

        let rejected = ValidationResult::rejected("missing required launcher path");
        assert_eq!(rejected.state, ValidationState::Rejected);
    }

    #[test]
    fn extraction_plan_enforces_boundary() {
        let boundary = ContainmentPolicy::new("/tmp/staging").unwrap();
        let plan = ExtractionPlan::new(
            "/tmp/release.zip",
            &boundary,
            vec![
                PathBuf::from("koreader/run.sh"),
                PathBuf::from("koreader/frontend/ui.lua"),
            ],
        )
        .unwrap();

        assert_eq!(plan.targets.len(), 2);
        assert_eq!(
            plan.targets[0].destination.full_path,
            PathBuf::from("/tmp/staging/koreader/run.sh")
        );

        assert!(matches!(
            ExtractionPlan::new(
                "/tmp/release.zip",
                &boundary,
                vec![PathBuf::from("../escape")]
            ),
            Err(PayloadError::Safety(_))
        ));
    }
}
