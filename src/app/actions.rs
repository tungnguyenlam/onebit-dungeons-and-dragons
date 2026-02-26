use crate::app::{App, AppState, DialogContext};
use crate::game::items::crafting::CraftingSystem;

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
            crate::game::story::dialog::resolve(&tree, "root", &mut self.world_state)
        {
            self.transition(AppState::Dialog(DialogContext {
                npc_name: npc.name.clone(),
                tree,
                current_node: "root".into(),
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
            self.set_feedback("Not enough spell slots!");
            return;
        }

        if let Some(heal_dice) = &spell_def.heal {
            let amount = heal_dice.roll();
            self.player.heal(amount as u32);
            self.player.spell_slots[slot_idx] -= 1;
            self.set_feedback(&format!(
                "Casted {}! Healed for {} HP.",
                spell_def.name, amount
            ));
        } else {
            self.player.spell_slots[slot_idx] -= 1;
            self.set_feedback(&format!("Casted {}.", spell_def.name));
        }
    }

    pub fn craft_item(&mut self, recipe_id: &str) -> bool {
        let crafting = CraftingSystem::new(self.recipe_defs.clone());

        if let Some(result) = crafting.craft(recipe_id, &mut self.player.inventory) {
            let result_name = self
                .item_defs
                .get(&result)
                .map(|i| i.name.as_str())
                .unwrap_or(&result);
            self.set_feedback(&format!("Crafted {}!", result_name));
            true
        } else {
            self.set_feedback("Not enough ingredients!");
            false
        }
    }

    pub fn get_available_recipes(&self) -> Vec<String> {
        let crafting = CraftingSystem::new(self.recipe_defs.clone());
        crafting
            .get_available_recipes(&self.player.inventory)
            .into_iter()
            .map(|r| r.id.clone())
            .collect()
    }

    pub fn harvest_from_monster(&mut self, monster_id: &str) -> Option<String> {
        let harvest_table = [
            ("giant_spider", "spider_silk"),
            ("ignis_cinder_drake", "dragon_scale"),
            ("goblin_shaman", "poison_sac"),
            ("ember_wraith", "crystal_shard"),
        ];

        for (mob, item) in harvest_table {
            if monster_id.contains(mob) {
                self.player.inventory.add(item, 1);
                let item_name = self
                    .item_defs
                    .get(item)
                    .map(|i| i.name.as_str())
                    .unwrap_or(item);
                return Some(format!("Harvested {} from the corpse.", item_name));
            }
        }
        None
    }
}
