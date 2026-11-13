mod entity;
mod patterns;

pub use entity::{canonical_entity_stem, operation_matches_entity};
pub use patterns::classify_operation;

#[cfg(test)]
mod tests;
