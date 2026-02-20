/// Story system sub-modules.
///
/// This module has no dependency on any renderer or UI code.
///
/// Sub-modules enabled as milestones are completed:
///   Milestone 1 — world_state (flag/counter store)  ← this PR
///   Milestone 3 — quest, dialog, journal, events
///
/// See docs/gameplay/story.md and docs/tasks/backlog.md.
pub mod world_state;

pub use world_state::WorldState;

// Planned — uncomment when implemented:
// pub mod quest;
// pub mod dialog;
// pub mod journal;
// pub mod events;
