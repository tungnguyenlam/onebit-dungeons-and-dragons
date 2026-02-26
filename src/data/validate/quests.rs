use super::report::*;
use crate::data::loader::{load_quests, load_region};
use crate::data::types::{DialogTree, QuestDef, TriggerKind};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
pub fn validate_quests(quests: &HashMap<String, QuestDef>, report: &mut ValidationReport) {
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
