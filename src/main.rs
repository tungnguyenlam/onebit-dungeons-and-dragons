#![allow(dead_code)]
#![allow(unused_imports)]
/// Entry point.
///
/// Parses the `--mode tui|gui` CLI flag (defaults to `tui`), constructs the
/// appropriate renderer, and drives the main game loop.
///
/// Build examples:
///   cargo run                                    # TUI (default feature)
///   cargo run --features gui -- --mode gui       # GUI window
///   cargo run --features gui -- --mode tui       # TUI, GUI feature compiled
///   cargo run --no-default-features \
///             --features gui  -- --mode gui      # GUI-only binary
mod app;
mod data;
mod game;
mod renderer;
mod ui;

use anyhow::Result;
use app::App;
use clap::{Parser, ValueEnum};
use renderer::{ControlFlow, GameEvent, GameRenderer};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "dnd", about = "OneBit Dungeons & Dragons")]
struct Cli {
    /// Choose the rendering front-end.
    #[arg(long, default_value = "tui")]
    mode: LaunchMode,
    /// Validate content assets and exit (no renderer loop).
    #[arg(long, default_value_t = false)]
    validate_assets: bool,
    /// Validate a save file and exit.  Exits non-zero on structural errors.
    #[arg(long)]
    validate_save: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
enum LaunchMode {
    /// Terminal UI (Ratatui + Crossterm). Requires feature "tui".
    Tui,
    /// Windowed GUI (egui + eframe). Requires feature "gui".
    Gui,
}

// ---------------------------------------------------------------------------
// Main loop
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.validate_assets {
        let report = data::validate::validate_assets("assets")?;
        for warn in &report.warnings {
            eprintln!("[warn] {warn}");
        }
        if report.has_errors() {
            for err in &report.errors {
                eprintln!("[error] {err}");
            }
            anyhow::bail!(
                "asset validation failed with {} error(s)",
                report.errors.len()
            );
        }
        println!(
            "asset validation passed ({} warning(s))",
            report.warnings.len()
        );
        return Ok(());
    }

    if let Some(save_path) = &cli.validate_save {
        use game::save::validate_save_file;
        match validate_save_file(save_path) {
            Ok(report) => {
                for warn in &report.warnings {
                    eprintln!("[warn] {warn}");
                }
                println!(
                    "save validation passed ({} warning(s)): {}",
                    report.warnings.len(),
                    save_path.display()
                );
                return Ok(());
            }
            Err(e) => {
                anyhow::bail!("{e}");
            }
        }
    }

    let app = App::new();

    match cli.mode {
        LaunchMode::Tui => {
            #[cfg(feature = "tui")]
            {
                let renderer = ui::tui::TuiRenderer::new()?;
                run_loop(Box::new(renderer), app)
            }
            #[cfg(not(feature = "tui"))]
            {
                anyhow::bail!(
                    "This binary was compiled without the 'tui' feature. \
                     Rebuild with: cargo run --features tui"
                )
            }
        }

        LaunchMode::Gui => {
            #[cfg(feature = "gui")]
            {
                // eframe drives its own event loop; we hand off to it here.
                ui::gui::run(app)
            }
            #[cfg(not(feature = "gui"))]
            {
                anyhow::bail!(
                    "This binary was compiled without the 'gui' feature. \
                     Rebuild with: cargo run --features gui -- --mode gui"
                )
            }
        }
    }
}

/// Renderer-agnostic game loop.
/// Used by TUI (and could be used by a headless test renderer).
/// GUI uses eframe's own loop instead of calling this function.
pub fn run_loop(mut renderer: Box<dyn GameRenderer>, mut app: App) -> Result<()> {
    loop {
        renderer.render(&app)?;

        match renderer.poll_event()? {
            GameEvent::Quit => break,
            event => {
                if app.handle_event(event)? == ControlFlow::Exit {
                    break;
                }
            }
        }
    }
    renderer.teardown()
}
