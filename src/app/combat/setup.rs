use crate::app::samples::combatant_from_monster;
use crate::app::state::{AppState, CombatContext};
use crate::app::App;
use crate::game::{
    character::{conditions::Condition, Character},
    combat::{
        apply_damage, roll_attack, AttackProfile, CombatantState, DefenseProfile, HitType, RollMode,
    },
    dice::DiceExpr,
};
use std::collections::HashMap;

impl App {
    pub fn make_combat_context(&mut self) -> CombatContext {
        let (bonus, dmg_dice, _, _, _, _) = self.equipment_bonus_totals();
        let resistances = self.equipment_resistances();
        let immunities = self.equipment_immunities();
        let condition_immunities = self.equipment_condition_immunities();

        let armor_id = self.player.equipment.armor.as_deref();
        let armor_def = armor_id.and_then(|id| self.item_defs.get(id));
        let armor_tuple =
            armor_def.and_then(|d| d.armor.as_ref().map(|a| (a.base_ac, &a.armor_type)));

        let shield_equipped = self
            .player
            .equipment
            .off_hand
            .as_deref()
            .and_then(|id| self.item_defs.get(id))
            .map(|d| {
                d.armor
                    .as_ref()
                    .map(|a| a.armor_type == crate::data::types::ArmorType::Shield)
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let ac = crate::game::items::armor::armor_class(
            armor_tuple,
            shield_equipped,
            self.player.scores.dex_mod(),
        );

        let prof = self.player.proficiency_bonus();
        let str_mod = self.player.scores.str_mod() as i32;

        let mut p_dmg_dice = dmg_dice;
        p_dmg_dice.modifier += str_mod;

        let mut p_combatant = CombatantState::new(
            "player",
            self.player.name.clone(),
            true,
            self.player.max_hp,
            ac,
            30,
            self.player.scores.dex_mod() as i32,
            prof + str_mod + bonus,
            p_dmg_dice,
        );
        p_combatant.current_hp = self.player.current_hp;
        p_combatant.resistances = resistances;
        p_combatant.immunities = immunities;
        p_combatant.condition_immunities = condition_immunities;
        if crate::game::character::progression::has_extra_attack(
            self.player.main_class(),
            self.player.total_level,
        ) {
            p_combatant.action_slots.base_extra_attacks = 1;
        }

        // Find damage type of main hand weapon
        if let Some(weapon_id) = self.player.equipment.main_hand.as_deref() {
            if let Some(item) = self.item_defs.get(weapon_id) {
                if let Some(w) = &item.weapon {
                    p_combatant.damage_type = w.damage_type.clone();
                }
            }
        }

        let mut enemies = self.build_encounter_monsters(self.pending_encounter_monster.as_deref());
        self.pending_encounter_monster = None;

        let mut combatants = vec![p_combatant];

        // Add "guard_ally" if flag is set
        if self.world_state.flag("town_guard_trusted") {
            let mut guard = CombatantState::new(
                "guard_ally",
                "Town Guard",
                true,
                20,
                16,
                30,
                0,
                4,
                DiceExpr::new(1, 8, 2),
            );
            guard.damage_type = "piercing".into(); // assuming spear/crossbow
            combatants.push(guard);
        }

        combatants.append(&mut enemies);

        let seed = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            & 0x7fffffff) as u32;

        CombatContext {
            state: crate::game::combat::CombatState::new_with_seed(combatants, seed as u64),
            world_state: self.world_state.clone(),
            log: vec!["A new battle begins!".into()],
            seed,
            selected_enemy_id: None,
        }
    }
    pub fn build_encounter_monsters(&self, queued_monster: Option<&str>) -> Vec<CombatantState> {
        let mut out = Vec::new();
        let ids = if let Some(mid) = queued_monster {
            vec![mid]
        } else {
            vec!["goblin", "goblin"]
        };

        let mut seq: HashMap<&str, usize> = HashMap::new();
        for mid in ids {
            let Some(def) = self.monster_defs.get(mid) else {
                continue;
            };
            let n = seq.entry(mid).and_modify(|v| *v += 1).or_insert(1);
            let cid = format!("{}_{}", mid, *n);
            out.push(combatant_from_monster(
                &cid,
                def,
                self.settings.enemy_hp_multiplier,
            ));
        }
        out
    }
}
