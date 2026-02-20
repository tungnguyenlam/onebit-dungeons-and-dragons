/// Quest stage machine runtime.
use crate::{
    data::types::{DialogEffect, QuestDef},
    game::story::{journal::{Category, Journal}, world_state::WorldState},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestStatus {
    Active { stage_id: String },
    Completed { stage_id: String },
    Failed { stage_id: String },
}

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
        Self { defs: map, states: HashMap::new() }
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
            QuestStatus::Active { stage_id: start.id.clone() },
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
                        QuestStatus::Completed { stage_id: current_stage_id.clone() },
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
                    QuestStatus::Active { stage_id: next_stage.id.clone() },
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
}
