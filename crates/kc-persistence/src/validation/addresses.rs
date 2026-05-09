use kc_domain::Address;

use crate::PersistenceError;

use super::{validate_absolute_local_path, validate_address_projection};

pub fn validate_source_root_address(address: &Address) -> Result<(), PersistenceError> {
    match address {
        Address::LocalPath(path) => {
            validate_absolute_local_path("backup_manifest.source_root", path)
        }
        Address::ScopedPath { .. } | Address::Remote { .. } | Address::Logical { .. } => {
            validate_address_projection("backup_manifest.source_root", address).map(|_| ())
        }
    }
}
