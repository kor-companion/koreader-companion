use std::path::PathBuf;

use kc_domain::{Address, ContainmentPolicy};

use crate::PayloadError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionTarget {
    pub relative_path: PathBuf,
    pub destination: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionPlan {
    pub archive: Address,
    pub destination_root: Address,
    pub targets: Vec<ExtractionTarget>,
}

impl ExtractionPlan {
    pub fn new(
        archive: Address,
        boundary: &ContainmentPolicy,
        relative_paths: impl IntoIterator<Item = PathBuf>,
    ) -> Result<Self, PayloadError> {
        let mut targets = Vec::new();

        for relative_path in relative_paths {
            let destination = boundary.contain(&relative_path)?;
            targets.push(ExtractionTarget {
                relative_path,
                destination: Address::filesystem(destination.full_path),
            });
        }

        Ok(Self {
            archive,
            destination_root: Address::filesystem(boundary.root().to_path_buf()),
            targets,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use kc_domain::{Address, ContainmentPolicy};

    use crate::{ExtractionPlan, PayloadError};

    #[test]
    fn extraction_plan_uses_addresses_and_enforces_boundary() {
        let boundary = ContainmentPolicy::new("/tmp/staging").unwrap();
        let plan = ExtractionPlan::new(
            Address::filesystem("/tmp/release.zip"),
            &boundary,
            vec![
                PathBuf::from("koreader/run.sh"),
                PathBuf::from("koreader/frontend/ui.lua"),
            ],
        )
        .unwrap();

        assert_eq!(plan.targets.len(), 2);
        assert_eq!(
            plan.targets[0].destination,
            Address::filesystem("/tmp/staging/koreader/run.sh")
        );

        assert!(matches!(
            ExtractionPlan::new(
                Address::filesystem("/tmp/release.zip"),
                &boundary,
                vec![PathBuf::from("../escape")],
            ),
            Err(PayloadError::Safety(_))
        ));
    }
}
