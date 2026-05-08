use crate::PayloadError;

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
    pub checksum_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseMetadata {
    pub release_id: String,
    pub version: String,
    pub channel: ReleaseChannel,
    pub published_at: crate::Timestamp,
    pub fetched_at: crate::Timestamp,
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

#[cfg(test)]
mod tests {
    use crate::{
        select_artifact, ArtifactRule, PayloadError, ReleaseAsset, ReleaseChannel, ReleaseMetadata,
        Timestamp,
    };

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
                    checksum_hex: Some(
                        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                            .to_string(),
                    ),
                },
                ReleaseAsset {
                    name: "koreader-remarkable-v2026.04.zip".to_string(),
                    download_url: "https://example.invalid/assets/rm.zip".to_string(),
                    size_bytes: 11,
                    content_type: Some("application/zip".to_string()),
                    checksum_hex: None,
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
    fn rejects_ambiguous_missing_or_disallowed_prerelease_artifacts() {
        let release = ReleaseMetadata {
            assets: vec![
                ReleaseAsset {
                    name: "koreader-kobo-a.zip".to_string(),
                    download_url: "a".to_string(),
                    size_bytes: 1,
                    content_type: None,
                    checksum_hex: None,
                },
                ReleaseAsset {
                    name: "koreader-kobo-b.zip".to_string(),
                    download_url: "b".to_string(),
                    size_bytes: 1,
                    content_type: None,
                    checksum_hex: None,
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
                    ..rule.clone()
                }
            ),
            Err(PayloadError::ArtifactNotFound { .. })
        ));
        assert!(matches!(
            select_artifact(
                &ReleaseMetadata {
                    channel: ReleaseChannel::Prerelease,
                    ..sample_release()
                },
                &rule,
            ),
            Err(PayloadError::PrereleaseNotAllowed(_))
        ));
    }
}
