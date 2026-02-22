use crate::{
    app::AppState,
    game::{
        character::Character,
        story::{Journal, WorldState},
    },
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use super::types::*;
#[derive(Debug, Default)]
pub struct SaveDriftReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl SaveDriftReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }

    fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}

/// Check whether a loaded `SaveGame` satisfies all structural invariants.
/// Returns a report; call `report.has_errors()` to decide whether to abort.
pub fn check_save_invariants(save: &SaveGame) -> SaveDriftReport {
    let mut report = SaveDriftReport::default();

    // Version bounds
    if save.format_version == 0 {
        report.warn("save has legacy format_version 0; treated as version 1");
    } else if save.format_version > SAVE_FORMAT_MAX_VERSION {
        report.error(format!(
            "save format_version {} exceeds maximum known version {}",
            save.format_version, SAVE_FORMAT_MAX_VERSION
        ));
    }

    // Player invariants
    let p = &save.player;
    if p.name.is_empty() {
        report.error("player name is empty");
    }
    if p.class_id.is_empty() {
        report.error("player class_id is empty");
    }
    if p.race_id.is_empty() {
        report.error("player race_id is empty");
    }
    if p.max_hp == 0 {
        report.error("player max_hp is 0");
    }
    if p.current_hp > p.max_hp {
        report.error(format!(
            "player current_hp ({}) exceeds max_hp ({})",
            p.current_hp, p.max_hp
        ));
    }
    if p.level == 0 {
        report.error("player level is 0");
    }
    if p.level > 20 {
        report.error(format!("player level {} exceeds 20", p.level));
    }
    for (i, (&slot, &max)) in p
        .spell_slots
        .iter()
        .zip(p.spell_slots_max.iter())
        .enumerate()
    {
        if slot > max {
            report.error(format!(
                "spell_slots[{i}] ({slot}) exceeds spell_slots_max[{i}] ({max})"
            ));
        }
    }

    // World state invariants
    if save.region_slug.is_empty() {
        report.error("region_slug is empty");
    }
    if save.room_id.is_empty() {
        report.error("room_id is empty");
    }

    // Position sanity (pos must fit in a reasonable map)
    if save.player_pos.0 > 512 || save.player_pos.1 > 512 {
        report.warn(format!(
            "player_pos ({}, {}) is unusually large",
            save.player_pos.0, save.player_pos.1
        ));
    }

    // Turn monotonicity hint
    if save.turn == 0 {
        report.warn("turn counter is 0 — save may be from before first tick");
    }

    report
}