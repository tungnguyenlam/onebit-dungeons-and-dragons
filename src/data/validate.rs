use crate::data::loader::{load_quests, load_region};
use crate::data::types::{DialogTree, QuestDef, TriggerKind};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

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
                        report.errors.push(format!(
                            "region '{slug}' room '{room_id}' has travel trigger to missing room '{}'",
                            trig.target_id
                        ));
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
                if seen.insert(t.target_id.clone()) {
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

fn validate_dialog(slug: &str, npc_id: &str, tree: &DialogTree, report: &mut ValidationReport) {
    let mut ids = HashSet::new();
    for node in &tree.nodes {
        if !ids.insert(node.id.clone()) {
            report.errors.push(format!(
                "region '{slug}' dialog '{npc_id}' has duplicate node id '{}'",
                node.id
            ));
        }
    }
    if !ids.contains("root") {
        report.errors.push(format!(
            "region '{slug}' dialog '{npc_id}' missing required 'root' node"
        ));
    }

    for node in &tree.nodes {
        if node.text == "__SKILL_CHECK__" {
            for target in [node.on_pass.as_deref(), node.on_fail.as_deref()]
                .into_iter()
                .flatten()
            {
                if target != "END" && !ids.contains(target) {
                    report.errors.push(format!(
                        "region '{slug}' dialog '{npc_id}' skill node '{}' points to missing node '{}'",
                        node.id, target
                    ));
                }
            }
        }
        for choice in &node.choices {
            if choice.next != "END" && !ids.contains(&choice.next) {
                report.errors.push(format!(
                    "region '{slug}' dialog '{npc_id}' node '{}' choice '{}' points to missing node '{}'",
                    node.id, choice.text, choice.next
                ));
            }
        }
    }
}

fn validate_quests(quests: &HashMap<String, QuestDef>, report: &mut ValidationReport) {
    for (quest_id, quest) in quests {
        if quest.stages.is_empty() {
            report
                .errors
                .push(format!("quest '{quest_id}' has no stages"));
            continue;
        }

        let mut stage_ids: HashSet<String> = HashSet::new();
        for stage in &quest.stages {
            if !stage_ids.insert(stage.id.clone()) {
                report.errors.push(format!(
                    "quest '{quest_id}' has duplicate stage id '{}'",
                    stage.id
                ));
            }
        }

        // Reachability graph ignoring condition truth values.
        let mut reachable: HashSet<String> = HashSet::new();
        let mut q = VecDeque::new();
        let start = quest.stages[0].id.clone();
        reachable.insert(start.clone());
        q.push_back(start);
        while let Some(id) = q.pop_front() {
            if let Some(stage) = quest.stages.iter().find(|s| s.id == id) {
                for tr in &stage.next {
                    if tr.stage == "DONE" || tr.stage == "END" {
                        continue;
                    }
                    if !stage_ids.contains(&tr.stage) {
                        report.errors.push(format!(
                            "quest '{quest_id}' stage '{}' transitions to missing stage '{}'",
                            stage.id, tr.stage
                        ));
                        continue;
                    }
                    if reachable.insert(tr.stage.clone()) {
                        q.push_back(tr.stage.clone());
                    }
                }
            }
        }

        for stage in &quest.stages {
            if !reachable.contains(&stage.id) {
                report.errors.push(format!(
                    "quest '{quest_id}' stage '{}' is unreachable from start stage '{}'",
                    stage.id, quest.stages[0].id
                ));
            }
            if stage.next.is_empty() {
                report.warnings.push(format!(
                    "quest '{quest_id}' stage '{}' has no outgoing transitions",
                    stage.id
                ));
            }
        }
    }
}

fn parse_grid_rows(grid: &str) -> Vec<Vec<char>> {
    grid.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn is_passable(tile: char) -> bool {
    matches!(tile, '.' | '-' | '~' | '^' | 'v' | 'X' | '@' | '!')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validator_passes_repo_assets() {
        let report = validate_assets("assets").expect("validator should run");
        assert!(
            !report.has_errors(),
            "unexpected validation errors: {:?}",
            report.errors
        );
    }

    #[test]
    fn all_regions_have_multiple_rooms() {
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            assert!(
                loaded.manifest.rooms.len() >= 2,
                "region '{slug}' must have >=2 rooms, has {}",
                loaded.manifest.rooms.len()
            );
        }
    }

    #[test]
    fn all_rooms_reachable_from_entry() {
        use crate::data::types::TriggerKind;
        use std::collections::{HashSet, VecDeque};
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            let mut seen: HashSet<String> = HashSet::new();
            let mut q: VecDeque<String> = VecDeque::new();
            seen.insert(loaded.manifest.entry_room.clone());
            q.push_back(loaded.manifest.entry_room.clone());
            while let Some(rid) = q.pop_front() {
                if let Some(room) = loaded.rooms.get(&rid) {
                    for t in room
                        .triggers
                        .iter()
                        .filter(|t| t.kind == TriggerKind::Travel)
                    {
                        if seen.insert(t.target_id.clone()) {
                            q.push_back(t.target_id.clone());
                        }
                    }
                }
            }
            for room_id in loaded.rooms.keys() {
                assert!(
                    seen.contains(room_id),
                    "[{slug}] room '{room_id}' unreachable"
                );
            }
        }
    }

    #[test]
    fn regions_have_branching_paths() {
        use crate::data::types::TriggerKind;
        let regions_dir = std::path::Path::new("assets/regions");
        for entry in std::fs::read_dir(regions_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = crate::data::loader::load_region("assets", &slug).unwrap();
            let branching = loaded.rooms.values().any(|room| {
                room.triggers
                    .iter()
                    .filter(|t| t.kind == TriggerKind::Travel)
                    .count()
                    >= 2
            });
            assert!(branching, "[{slug}] needs >=1 room with 2+ travel triggers");
        }
    }
}
