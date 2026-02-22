use crate::data::loader::{load_quests, load_region};
use crate::data::types::{DialogTree, QuestDef, TriggerKind};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use super::report::*;
pub fn parse_grid_rows(grid: &str) -> Vec<Vec<char>> {
    grid.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

pub fn is_passable(tile: char) -> bool {
    matches!(tile, '.' | '-' | ',' | '^' | 'v' | 'X' | '@' | '!')
}