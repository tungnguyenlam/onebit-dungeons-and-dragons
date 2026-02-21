/// Quest stage machine runtime.
use crate::{
    data::types::{DialogEffect, QuestDef},
    game::story::{
        journal::{Category, Journal},
        world_state::WorldState,
    },
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestStatus {
    Active { stage_id: String },
    Completed { stage_id: String },
    Failed { stage_id: String },
}

/// Reason a quest stage cannot advance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockedReason {
    /// The active stage exists but none of its transitions are satisfied.
    NoSatisfiedTransition,
    /// The active stage references a stage id that doesn't exist in the definition.
    MissingStage(String),
    /// The quest definition itself is missing from the log.
    MissingDef,
}

/// Diagnostic snapshot for a blocked quest.
#[derive(Debug, Clone)]
pub struct QuestBlockedDiag {
    pub quest_id: String,
    pub stage_id: String,
    pub reason: BlockedReason,
}

// ---------------------------------------------------------------------------
// QuestLog
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct QuestLog {
    pub defs: HashMap<String, QuestDef>,
    pub states: HashMap<String, QuestStatus>,
}

impl QuestLog {
    pub fn with_defs(defs: impl IntoIterator<Item = QuestDef>) -> Self {
        let mut map = HashMap::new();
        for def in defs {
            map.insert(def.id.clone(), def);
        }
        Self {
            defs: map,
            states: HashMap::new(),
        }
    }

    pub fn accept_quest(
        &mut self,
        quest_id: &str,
        world: &mut WorldState,
        journal: &mut Journal,
        turn: u64,
    ) -> bool {
        if self.states.contains_key(quest_id) {
            return false;
        }
        let Some(def) = self.defs.get(quest_id) else {
            return false;
        };
        let Some(start) = def.stages.first() else {
            return false;
        };

        world.set_flag(format!("quest_{quest_id}_active"));
        self.states.insert(
            quest_id.to_string(),
            QuestStatus::Active {
                stage_id: start.id.clone(),
            },
        );
        apply_effects(&start.on_enter, world);
        journal.append(
            format!("quest-{quest_id}-{}", start.id),
            turn,
            Category::Quest,
            Some(quest_id.to_string()),
            format!("Quest: {}", def.name),
            start.journal_entry.clone(),
        );
        true
    }

    pub fn active_stage<'a>(&'a self, quest_id: &str) -> Option<&'a str> {
        match self.states.get(quest_id) {
            Some(QuestStatus::Active { stage_id }) => Some(stage_id.as_str()),
            _ => None,
        }
    }

    pub fn tick_quest(
        &mut self,
        quest_id: &str,
        world: &mut WorldState,
        journal: &mut Journal,
        turn: u64,
    ) -> bool {
        let Some(def) = self.defs.get(quest_id).cloned() else {
            return false;
        };
        let Some(current_stage_id) = self.active_stage(quest_id).map(str::to_string) else {
            return false;
        };
        let Some(current) = def.stages.iter().find(|s| s.id == current_stage_id) else {
            return false;
        };

        for t in &current.next {
            if world.evaluate(&t.condition) {
                if t.stage == "END" || t.stage == "DONE" {
                    self.states.insert(
                        quest_id.to_string(),
                        QuestStatus::Completed {
                            stage_id: current_stage_id.clone(),
                        },
                    );
                    world.clear_flag(format!("quest_{quest_id}_active"));
                    world.set_flag(format!("quest_{quest_id}_completed"));
                    return true;
                }
                let Some(next_stage) = def.stages.iter().find(|s| s.id == t.stage) else {
                    continue;
                };
                self.states.insert(
                    quest_id.to_string(),
                    QuestStatus::Active {
                        stage_id: next_stage.id.clone(),
                    },
                );
                apply_effects(&next_stage.on_enter, world);
                journal.append(
                    format!("quest-{quest_id}-{}", next_stage.id),
                    turn,
                    Category::Quest,
                    Some(quest_id.to_string()),
                    format!("Quest: {}", def.name),
                    next_stage.journal_entry.clone(),
                );
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self, world: &mut WorldState, journal: &mut Journal, turn: u64) {
        let active_ids: Vec<String> = self.states.iter()
            .filter_map(|(id, status)| {
                if matches!(status, QuestStatus::Active { .. }) {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();
        for _id in active_ids {
            self.tick_quest(&_id, world, journal, turn);
        }
    }

    // -----------------------------------------------------------------------
    // M22: diagnostics + recovery
    // -----------------------------------------------------------------------

    /// Return diagnostics for every active quest that cannot advance.
    ///
    /// A quest is "blocked" when its active stage has transitions defined but
    /// none are currently satisfiable — usually because a required world-state
    /// flag was never set.  This does NOT consider quests whose current stage
    /// has NO transitions at all (those are terminal stages, not stuck quests).
    pub fn blocked_quests(&self, world: &WorldState) -> Vec<QuestBlockedDiag> {
        let mut out = Vec::new();
        for (quest_id, status) in &self.states {
            let QuestStatus::Active { stage_id } = status else {
                continue;
            };
            let Some(def) = self.defs.get(quest_id.as_str()) else {
                out.push(QuestBlockedDiag {
                    quest_id: quest_id.clone(),
                    stage_id: stage_id.clone(),
                    reason: BlockedReason::MissingDef,
                });
                continue;
            };
            let Some(stage) = def.stages.iter().find(|s| s.id == *stage_id) else {
                out.push(QuestBlockedDiag {
                    quest_id: quest_id.clone(),
                    stage_id: stage_id.clone(),
                    reason: BlockedReason::MissingStage(stage_id.clone()),
                });
                continue;
            };
            // A stage with no `next` entries is terminal — not blocked.
            if stage.next.is_empty() {
                continue;
            }
            // Has transitions but none are satisfied right now.
            let any_satisfied = stage.next.iter().any(|t| world.evaluate(&t.condition));
            if !any_satisfied {
                out.push(QuestBlockedDiag {
                    quest_id: quest_id.clone(),
                    stage_id: stage_id.clone(),
                    reason: BlockedReason::NoSatisfiedTransition,
                });
            }
        }
        out
    }

    /// Write a journal recovery hint for every currently blocked quest.
    ///
    /// Emits one `Category::System` entry per blocked quest so the player
    /// gets actionable feedback in the log.  Returns the number of hints added.
    pub fn emit_blocked_hints(
        &mut self,
        world: &WorldState,
        journal: &mut Journal,
        turn: u64,
    ) -> usize {
        let blocked = self.blocked_quests(world);
        let count = blocked.len();
        for diag in &blocked {
            let message = match &diag.reason {
                BlockedReason::NoSatisfiedTransition => format!(
                    "[Quest hint] '{}' is stuck at stage '{}'. Check if you have completed the required objectives.",
                    diag.quest_id, diag.stage_id
                ),
                BlockedReason::MissingStage(s) => format!(
                    "[Quest error] '{}' references unknown stage '{s}'. This may be a content bug.",
                    diag.quest_id
                ),
                BlockedReason::MissingDef => format!(
                    "[Quest error] Quest '{}' is active but its definition is missing.",
                    diag.quest_id
                ),
            };
            journal.append(
                format!("quest-hint-{}-{}", diag.quest_id, turn),
                turn,
                Category::System,
                Some(diag.quest_id.clone()),
                "Quest Hint".to_string(),
                message,
            );
        }
        count
    }
}

fn apply_effects(effects: &[DialogEffect], world: &mut WorldState) {
    for effect in effects {
        match effect {
            DialogEffect::SetFlag { set_flag } => world.set_flag(set_flag.clone()),
            DialogEffect::ClearFlag { clear_flag } => world.clear_flag(clear_flag.clone()),
            DialogEffect::DeltaCounter { delta_counter } => {
                world.delta_counter(delta_counter.key.clone(), delta_counter.delta);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{QuestKind, QuestStageDef, QuestTransition};

    fn sample() -> QuestDef {
        QuestDef {
            id: "q1".into(),
            name: "Test Quest".into(),
            kind: QuestKind::Side,
            stages: vec![
                QuestStageDef {
                    id: "start".into(),
                    label: "Start".into(),
                    condition: "".into(),
                    on_enter: vec![],
                    next: vec![QuestTransition {
                        condition: "flag:go".into(),
                        stage: "track".into(),
                    }],
                    journal_entry: "Started".into(),
                },
                QuestStageDef {
                    id: "track".into(),
                    label: "Track".into(),
                    condition: "flag:go".into(),
                    on_enter: vec![],
                    next: vec![QuestTransition {
                        condition: "flag:done".into(),
                        stage: "DONE".into(),
                    }],
                    journal_entry: "Tracking".into(),
                },
            ],
        }
    }

    #[test]
    fn accept_sets_stage_and_flag() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        assert!(log.accept_quest("q1", &mut ws, &mut j, 1));
        assert_eq!(log.active_stage("q1"), Some("start"));
        assert!(ws.flag("quest_q1_active"));
    }

    #[test]
    fn tick_advances_stage_by_condition() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        ws.set_flag("go");
        assert!(log.tick_quest("q1", &mut ws, &mut j, 2));
        assert_eq!(log.active_stage("q1"), Some("track"));
    }

    #[test]
    fn done_transition_marks_completed() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        ws.set_flag("go");
        log.tick_quest("q1", &mut ws, &mut j, 2);
        ws.set_flag("done");
        assert!(log.tick_quest("q1", &mut ws, &mut j, 3));
        assert!(matches!(
            log.states.get("q1"),
            Some(QuestStatus::Completed { .. })
        ));
        assert!(ws.flag("quest_q1_completed"));
    }

    // M22 tests ---------------------------------------------------------------

    #[test]
    fn blocked_quest_detected_when_no_transition_satisfied() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        // "go" flag NOT set → transition cannot fire → quest is blocked
        let blocked = log.blocked_quests(&ws);
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].quest_id, "q1");
        assert_eq!(blocked[0].reason, BlockedReason::NoSatisfiedTransition);
    }

    #[test]
    fn not_blocked_when_transition_satisfiable() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        ws.set_flag("go"); // satisfies the transition
        let blocked = log.blocked_quests(&ws);
        assert!(blocked.is_empty());
    }

    #[test]
    fn completed_quest_not_reported_as_blocked() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        ws.set_flag("go");
        log.tick_quest("q1", &mut ws, &mut j, 2);
        ws.set_flag("done");
        log.tick_quest("q1", &mut ws, &mut j, 3);
        let blocked = log.blocked_quests(&ws);
        assert!(blocked.is_empty());
    }

    #[test]
    fn emit_blocked_hints_adds_journal_entries() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        // Don't set "go" → quest is stuck
        let count = log.emit_blocked_hints(&ws, &mut j, 2);
        assert_eq!(count, 1);
        // Journal should have an entry tagged System for the hint
        let entries: Vec<_> = j.entries().collect();
        let has_hint = entries
            .iter()
            .any(|e| e.category == Category::System && e.body.contains("stuck"));
        assert!(has_hint);
    }

    #[test]
    fn no_hints_emitted_when_no_blocked_quests() {
        let mut log = QuestLog::with_defs(vec![sample()]);
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        log.accept_quest("q1", &mut ws, &mut j, 1);
        ws.set_flag("go"); // not blocked
        let before = j.entries().count();
        let count = log.emit_blocked_hints(&ws, &mut j, 2);
        assert_eq!(count, 0);
        assert_eq!(j.entries().count(), before);
    }
}
