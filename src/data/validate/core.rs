use crate::data::loader::{load_quests, load_region};
use crate::data::types::{DialogTree, QuestDef, TriggerKind};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use super::report::*;
use super::dialog::*;
use super::quests::*;
use super::utils::*;
pub fn validate_assets(base: impl AsRef<Path>) -> Result<ValidationReport> {
    let base = base.as_ref();
    let mut report = ValidationReport::default();

    let regions_dir = base.join("regions");
    if regions_dir.is_dir() {
        for entry in fs::read_dir(&regions_dir)? {
            let entry = entry?;
            if !entry.path().is_dir() {
                continue;
            }
            let slug = entry.file_name().to_string_lossy().to_string();
            validate_region(base, &slug, &mut report)?;
        }
    }

    let quests = load_quests(base).unwrap_or_default();
    validate_quests(&quests, &mut report);

    Ok(report)
}

fn validate_region(base: &Path, slug: &str, report: &mut ValidationReport) -> Result<()> {
    let loaded = load_region(base, slug)?;

    if loaded.manifest.rooms.is_empty() {
        report
            .errors
            .push(format!("region '{slug}' has no rooms in manifest"));
        return Ok(());
    }

    // Warn if region has fewer than 2 rooms (recommended minimum).
    if loaded.manifest.rooms.len() < 2 {
        report.warnings.push(format!(
            "region '{slug}' has only {} room(s); recommended minimum is 2",
            loaded.manifest.rooms.len()
        ));
    }

    if !loaded.rooms.contains_key(&loaded.manifest.entry_room) {
        report.errors.push(format!(
            "region '{slug}' entry room '{}' missing from loaded rooms",
            loaded.manifest.entry_room
        ));
    }

    for (room_id, room) in &loaded.rooms {
        let rows = parse_grid_rows(&room.grid);
        let height = rows.len();
        let width = rows.first().map(|r| r.len()).unwrap_or(0);

        let mut travel_count = 0usize;
        for trig in &room.triggers {
            let col = trig.position[0] as usize;
            let row = trig.position[1] as usize;
            if row >= height || col >= width {
                report.errors.push(format!(
                    "region '{slug}' room '{room_id}' trigger {:?} at [{}, {}] is out of bounds",
                    trig.kind, trig.position[0], trig.position[1]
                ));
                continue;
            }

            let tile = rows[row][col];
            if !is_passable(tile) {
                report.errors.push(format!(
                    "region '{slug}' room '{room_id}' trigger {:?} at [{}, {}] sits on blocked tile '{}'",
                    trig.kind, trig.position[0], trig.position[1], tile
                ));
            }

            match trig.kind {
                TriggerKind::Travel => {
                    travel_count += 1;
                    if !loaded.rooms.contains_key(&trig.target_id) {
                        let is_external =
                            loaded.manifest.connections.iter().any(|c| {
                                c.to_region == trig.target_id || c.to_room == trig.target_id
                            });
                        if !is_external {
                            report.errors.push(format!(
                                "region '{slug}' room '{room_id}' has travel trigger to missing room '{}'",
                                trig.target_id
                            ));
                        }
                    }
                }
                TriggerKind::Dialog => {
                    if !loaded.npcs.contains_key(&trig.target_id) {
                        report.errors.push(format!(
                            "region '{slug}' room '{room_id}' dialog trigger references missing npc '{}'",
                            trig.target_id
                        ));
                    }
                }
                TriggerKind::Lore | TriggerKind::Encounter | TriggerKind::QuestStage => {}
            }
        }

        // Terminal rooms are explicitly allowed to have no outbound travel.
        if travel_count == 0 && !room.terminal {
            report.errors.push(format!(
                "region '{slug}' room '{room_id}' has no outbound travel trigger (mark terminal=true to allow)"
            ));
        }
    }

    // Reachability from entry via travel triggers.
    let mut seen: HashSet<String> = HashSet::new();
    let mut q = VecDeque::new();
    seen.insert(loaded.manifest.entry_room.clone());
    q.push_back(loaded.manifest.entry_room.clone());
    while let Some(room_id) = q.pop_front() {
        if let Some(room) = loaded.rooms.get(&room_id) {
            for t in room
                .triggers
                .iter()
                .filter(|t| t.kind == TriggerKind::Travel)
            {
                if loaded.rooms.contains_key(&t.target_id) && seen.insert(t.target_id.clone()) {
                    q.push_back(t.target_id.clone());
                }
            }
        }
    }
    for room_id in loaded.rooms.keys() {
        if !seen.contains(room_id) {
            report.errors.push(format!(
                "region '{slug}' room '{room_id}' is unreachable from entry '{}'",
                loaded.manifest.entry_room
            ));
        }
    }

    for (npc_id, tree) in &loaded.dialogs {
        validate_dialog(slug, npc_id, tree, report);
    }

    Ok(())
}