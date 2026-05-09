mod addresses;
mod paths;
mod projections;

pub use addresses::validate_source_root_address;
pub use paths::{derive_source_relative_path, validate_absolute_local_path};
pub use projections::validate_address_projection;
