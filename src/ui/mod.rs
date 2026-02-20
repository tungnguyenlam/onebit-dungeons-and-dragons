/// UI sub-modules — one per renderer back-end.
///
/// Neither sub-module imports from `src/game/`. Both implement the
/// `GameRenderer` trait defined in `src/renderer.rs`.
#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "gui")]
pub mod gui;
