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
use super::validate::*;
pub fn save_to_path(path: impl AsRef<Path>, save: &SaveGame) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating save directory {}", parent.display()))?;
    }
    let data = toml::to_string_pretty(save).context("serializing save file")?;
    std::fs::write(path, data).with_context(|| format!("writing save file {}", path.display()))?;
    Ok(())
}

pub fn load_from_path(path: impl AsRef<Path>) -> Result<SaveGame> {
    let path = path.as_ref();
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("reading save file {}", path.display()))?;
    let mut save: SaveGame = toml::from_str(&data).context("parsing save file TOML")?;
    if save.format_version == 0 {
        // Legacy saves missing version metadata — normalise to v1.
        save.format_version = super::types::SAVE_FORMAT_VERSION;
    }
    Ok(save)
}

/// Load a save file and run invariant checks.
/// Prints warnings; returns `Err` if structural errors are found.
pub fn validate_save_file(path: impl AsRef<Path>) -> Result<SaveDriftReport> {
    let path = path.as_ref();
    let save = load_from_path(path)?;
    let report = check_save_invariants(&save);
    if report.has_errors() {
        bail!(
            "save file '{}' failed invariant checks: {}",
            path.display(),
            report.errors.join("; ")
        );
    }
    Ok(report)
}