/// Story system sub-modules.
///
/// This module has no dependency on any renderer or UI code.
///
/// Sub-modules enabled as milestones are completed:
///   Milestone 1 — world_state (flag/counter store)
///   Milestone 3 — quest, dialog, journal, events
///
/// See docs/gameplay/story.md and docs/tasks/backlog.md.
pub mod dialog;
pub mod events;
pub mod journal;
pub mod quest;
pub mod world_state;

pub use journal::Journal;
// pub use quest::{QuestLog, QuestStatus};
pub use world_state::WorldState;
