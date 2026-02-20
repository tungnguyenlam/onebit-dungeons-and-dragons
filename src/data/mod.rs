/// Asset data access layer.
///
/// Exposes typed TOML structs (`types`) and loading utilities (`loader`).
/// Nothing in this module contains game logic.
pub mod loader;
pub mod types;
pub mod validate;

pub use loader::{GlobalAssets, LoadedRegion};
