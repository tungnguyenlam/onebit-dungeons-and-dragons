use crate::app::{App, AppState, DialogContext};

impl App {
    pub fn start_dialog_with_npc(&mut self, npc_id: &str) {
        let Some(npc) = self.region_npcs.get(npc_id) else {
            return;
        };
        let tree = if !npc.dialog_ref.is_empty() {
            self.region_dialogs.get(&npc.dialog_ref).cloned()
        } else {
            self.region_dialogs.get(npc_id).cloned()
        };

        let Some(tree) = tree else { return };

        if let Some(resolved) =
            crate::game::story::dialog::resolve(&tree, "START", &mut self.world_state)
        {
            self.transition(AppState::Dialog(DialogContext {
                npc_name: npc.name.clone(),
                tree,
                current_node: "START".into(),
                resolved,
            }));
        }
    }

    pub fn use_healing_potion(&mut self) {
        if self.player.inventory.count("healing_potion") > 0 {
            self.player.inventory.remove("healing_potion", 1);
            self.player.current_hp = (self.player.current_hp + 10).min(self.player.max_hp);
        }
    }

    pub fn cast_known_spell(&mut self, idx: usize) {
        let Some(spell_id) = self.known_spells.get(idx).cloned() else {
            return;
        };
        let Some(spell_def) = self.spell_defs.get(&spell_id) else {
            return;
        };

        let slot_idx = (spell_def.level.saturating_sub(1)) as usize;
        if slot_idx >= 9 || self.player.spell_slots[slot_idx] == 0 {
            return;
        }

        if let Some(heal_dice) = &spell_def.heal {
            let amount = heal_dice.roll();
            self.player.heal(amount as u32);
            self.player.spell_slots[slot_idx] -= 1;
        }
    }
}
