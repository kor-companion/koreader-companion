mod errors;
mod normalize;
mod policy;

#[cfg(test)]
mod tests;

pub use errors::SafetyViolation;
pub use policy::{ContainedPath, ContainmentPolicy};
