use super::report::*;
use crate::data::loader::{load_quests, load_region};
use crate::data::types::{DialogTree, QuestDef, TriggerKind};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
pub fn validate_dialog(slug: &str, npc_id: &str, tree: &DialogTree, report: &mut ValidationReport) {
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
