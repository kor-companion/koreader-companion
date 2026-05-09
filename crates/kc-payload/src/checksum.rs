use crate::PayloadError;

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

#[cfg(test)]
mod tests {
    use super::{Checksum, ChecksumAlgorithm};

    #[test]
    fn checksums_validate_hex_shape_and_match_case_insensitively() {
        let checksum = Checksum::new(
            ChecksumAlgorithm::Sha256,
            "abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        )
        .unwrap();

        assert!(
            checksum.matches("ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCD")
        );
        assert!(Checksum::new(ChecksumAlgorithm::Sha256, "not-hex").is_err());
    }
}
