/// Game logic sub-systems.
///
/// This module has **no** dependency on ratatui, crossterm, egui, or eframe.
/// It is tested independently from any renderer.
///
/// Sub-modules are enabled as milestones are completed:
///   Milestone 1 — dice, character, items, world (in progress)
///   Milestone 2 — combat
///   Milestone 3 — story, dialog
///   Milestone 5 — npc, ai
///
/// See docs/tasks/backlog.md for the full plan.
pub mod character;
pub mod dice;
pub mod items;

// Planned — uncomment when implemented:
// pub mod world;
// pub mod combat;
// pub mod story;
// pub mod npc;
// pub mod save;
