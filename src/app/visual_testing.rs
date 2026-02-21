use crate::app::{App, AppState};
use crate::renderer::GameRenderer;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct VisualCapture {
    pub frame_buffer: String,
    pub width: u16,
    pub height: u16,
}

pub struct VisualRegressionEngine {
    captures: HashMap<String, VisualCapture>,
    baselines: HashMap<String, String>,
    diffs: Vec<VisualDiff>,
}

#[derive(Debug, Clone)]
pub struct VisualDiff {
    pub screen: String,
    pub expected: String,
    pub actual: String,
    pub diff_count: usize,
    pub line_diffs: Vec<LineDiff>,
}

#[derive(Debug, Clone)]
pub struct LineDiff {
    pub line_number: usize,
    pub expected: String,
    pub actual: String,
}

impl VisualRegressionEngine {
    pub fn new() -> Self {
        Self {
            captures: HashMap::new(),
            baselines: HashMap::new(),
            diffs: Vec::new(),
        }
    }

    pub fn load_baselines(&mut self, baseline_dir: &PathBuf) -> Result<()> {
        if !baseline_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(baseline_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "txt") {
                let screen_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                let content = std::fs::read_to_string(&path)?;
                self.baselines.insert(screen_name, content);
            }
        }
        Ok(())
    }

    pub fn capture_screen(&mut self, screen_name: &str, app: &App) -> String {
        let frame = match &app.state {
            AppState::MainMenu => "main_menu",
            AppState::CharacterCreation => "character_creation",
            AppState::WorldMap => "world_map",
            AppState::Combat(_) => "combat",
            AppState::Dialog(_) => "dialog",
            AppState::Journal => "journal",
            AppState::Inventory => "inventory",
            AppState::Spellbook => "spellbook",
            AppState::Settings => "settings",
            AppState::GameOver => "game_over",
        };

        let key = format!("{}_{}", screen_name, frame);
        let capture = format!(
            "State: {:?}\nPlayer: {} (HP: {}/{})\nRoom: {}\nRegion: {}\nLevel: {} XP: {}\n",
            app.state,
            app.player.name,
            app.player.hp,
            app.player.max_hp(),
            app.current_room_id,
            app.region.manifest.slug,
            app.player.level,
            app.player.xp
        );

        self.captures.insert(
            key.clone(),
            VisualCapture {
                frame_buffer: capture.clone(),
                width: 80,
                height: 24,
            },
        );

        capture
    }

    pub fn compare(&mut self, screen_name: &str) -> Option<VisualDiff> {
        let key = screen_name.to_string();

        let actual = self.captures.get(&key)?.frame_buffer.clone();
        let expected = self.baselines.get(&key)?.clone();

        if actual == expected {
            return None;
        }

        let mut line_diffs = Vec::new();
        let actual_lines: Vec<&str> = actual.lines().collect();
        let expected_lines: Vec<&str> = expected.lines().collect();

        for (i, (exp, act)) in expected_lines.iter().zip(actual_lines.iter()).enumerate() {
            if exp != act {
                line_diffs.push(LineDiff {
                    line_number: i + 1,
                    expected: exp.to_string(),
                    actual: act.to_string(),
                });
            }
        }

        let diff_count = line_diffs.len();

        self.diffs.push(VisualDiff {
            screen: key.clone(),
            expected,
            actual,
            diff_count,
            line_diffs: line_diffs.clone(),
        });

        Some(VisualDiff {
            screen: key,
            expected: expected,
            actual,
            diff_count,
            line_diffs,
        })
    }

    pub fn save_baseline(&self, screen_name: &str, output_dir: &PathBuf) -> Result<()> {
        let capture = self
            .captures
            .get(screen_name)
            .ok_or_else(|| anyhow::anyhow!("No capture for {}", screen_name))?;

        let output_path = output_dir.join(format!("{}.txt", screen_name));
        std::fs::write(output_path, &capture.frame_buffer)?;
        Ok(())
    }

    pub fn get_diffs(&self) -> &[VisualDiff] {
        &self.diffs
    }

    pub fn has_diffs(&self) -> bool {
        !self.diffs.is_empty()
    }

    pub fn clear_captures(&mut self) {
        self.captures.clear();
        self.diffs.clear();
    }
}

impl Default for VisualRegressionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualDiff {
    pub fn report(&self) -> String {
        let mut report = format!(
            "Visual regression in '{}': {} line(s) differ\n",
            self.screen, self.diff_count
        );

        for diff in &self.line_diffs {
            report.push_str(&format!(
                "  Line {}:\n    Expected: {}\n    Actual:   {}\n",
                diff.line_number, diff.expected, diff.actual
            ));
        }

        report
    }
}
