/// Emergent world events and lore inspection helpers.
use crate::{
    data::types::LoreEntry,
    game::story::{journal::{Category, Journal}, world_state::WorldState},
};

#[derive(Debug, Clone)]
pub enum WorldEvent {
    AddJournalEntry {
        id: String,
        category: Category,
        title: String,
        body: String,
    },
    SetFlag { key: String },
    DeltaCounter { key: String, delta: i32 },
}

#[derive(Debug, Clone)]
pub struct EventTrigger {
    pub condition: String,
    pub event: WorldEvent,
    pub once: bool,
    pub fired: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EventEngine {
    pub triggers: Vec<EventTrigger>,
}

impl EventEngine {
    pub fn tick(&mut self, world: &mut WorldState, journal: &mut Journal, turn: u64) {
        for t in &mut self.triggers {
            if t.once && t.fired {
                continue;
            }
            if !world.evaluate(&t.condition) {
                continue;
            }
            match &t.event {
                WorldEvent::AddJournalEntry { id, category, title, body } => {
                    journal.append(id.clone(), turn, *category, None, title.clone(), body.clone());
                }
                WorldEvent::SetFlag { key } => world.set_flag(key.clone()),
                WorldEvent::DeltaCounter { key, delta } => world.delta_counter(key.clone(), *delta),
            }
            if t.once {
                t.fired = true;
            }
        }
    }
}

/// Inspect a lore entry and append to journal once.
pub fn inspect_lore(
    lore: &LoreEntry,
    world: &mut WorldState,
    journal: &mut Journal,
    turn: u64,
) -> bool {
    let seen_key = format!("lore_seen:{}", lore.id);
    if world.flag(&seen_key) {
        return false;
    }
    world.set_flag(seen_key);
    journal.append(
        format!("lore-{}", lore.id),
        turn,
        Category::Lore,
        None,
        lore.title.clone(),
        lore.text.clone(),
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn once_trigger_fires_once() {
        let mut e = EventEngine {
            triggers: vec![EventTrigger {
                condition: "flag:a".into(),
                event: WorldEvent::DeltaCounter {
                    key: "x".into(),
                    delta: 1,
                },
                once: true,
                fired: false,
            }],
        };
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        ws.set_flag("a");
        e.tick(&mut ws, &mut j, 1);
        e.tick(&mut ws, &mut j, 2);
        assert_eq!(ws.counter("x"), 1);
    }

    #[test]
    fn lore_inspect_appends_once() {
        let lore = LoreEntry {
            id: "stone".into(),
            title: "Stone".into(),
            text: "Text".into(),
            tags: vec![],
        };
        let mut ws = WorldState::new();
        let mut j = Journal::default();
        assert!(inspect_lore(&lore, &mut ws, &mut j, 1));
        assert!(!inspect_lore(&lore, &mut ws, &mut j, 2));
        assert_eq!(j.entries.len(), 1);
    }
}
