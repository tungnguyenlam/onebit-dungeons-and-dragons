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
    /// Step-through debug mode: render one frame, process input, render again, exit.
    /// Useful for automated testing - each keypress is passed as a separate run.
    /// Usage: cargo run -- --step [key] - passes key to game, renders, then exits
    #[arg(long, default_value_t = false)]
    step: bool,
    /// Text dump mode: output game state as plain text (no TTY required).
    /// Combine with --step for step-through testing without terminal.
    /// Usage: cargo run -- --text          # Start game, dump state
    ///        cargo run -- --text --step n  # Press 'n', dump state
    #[arg(long, default_value_t = false)]
    text: bool,
    /// Key to press in step mode. Can be passed as argument instead of stdin.
    /// Example: cargo run -- --text --step -k j
    #[arg(short, default_value = "")]
    key: String,
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

    // Text mode - headless text dump (no TTY required)
    if cli.text {
        return run_text_mode(app, cli.step, &cli.key);
    }

    match cli.mode {
        LaunchMode::Tui => {
            #[cfg(feature = "tui")]
            {
                let renderer = ui::tui::TuiRenderer::new()?;
                if cli.step {
                    run_step_mode(Box::new(renderer), app)
                } else {
                    run_loop(Box::new(renderer), app)
                }
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

/// Text dump mode - headless testing without TTY.
/// Dumps the current game state as plain text to stdout.
pub fn run_text_mode(mut app: App, step: bool, key: &str) -> Result<()> {
    // If step mode, process one input first
    if step && !key.is_empty() {
        let event = char_to_game_event(key.chars().next().unwrap_or(' '));
        if let Some(event) = event {
            app.handle_event(event)?;
        }
    }

    // Dump current state
    println!("{}", app.dump_state());
    Ok(())
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

/// Step-through debug mode for testing.
/// Renders one frame, waits for a single keypress from stdin,
/// processes it, renders again, then exits.
/// This allows agents to test the game one input at a time.
pub fn run_step_mode(mut renderer: Box<dyn GameRenderer>, mut app: App) -> Result<()> {
    use std::io::Read;

    // Initial render
    renderer.render(&app)?;

    // Read single character from stdin
    let mut input = [0u8; 1];
    match std::io::stdin().read(&mut input) {
        Ok(1) => {
            let key = input[0] as char;
            let event = char_to_game_event(key);
            if let Some(event) = event {
                if app.handle_event(event)? == ControlFlow::Exit {
                    // Don't render on exit
                    return renderer.teardown();
                }
            }
        }
        Ok(0) => {
            eprintln!("[step] No input provided. Usage: cargo run -- --step <key>");
            return renderer.teardown();
        }
        Ok(_) => {
            eprintln!("[step] Warning: multiple bytes read, using first byte");
            let key = input[0] as char;
            let event = char_to_game_event(key);
            if let Some(event) = event {
                if app.handle_event(event)? == ControlFlow::Exit {
                    return renderer.teardown();
                }
            }
        }
        Err(e) => {
            eprintln!("[step] Error reading stdin: {e}");
            return renderer.teardown();
        }
    }

    // Render after processing input
    renderer.render(&app)?;
    renderer.teardown()
}

/// Convert a character to a GameEvent for step mode.
fn char_to_game_event(c: char) -> Option<GameEvent> {
    match c {
        // Navigation (vim-style or direct letters)
        'k' | 'K' => Some(GameEvent::MoveUp),
        'j' | 'J' => Some(GameEvent::MoveDown),
        'h' | 'H' => Some(GameEvent::MoveLeft),
        'l' | 'L' => Some(GameEvent::MoveRight),
        // Actions
        '\r' | '\n' | ' ' => Some(GameEvent::Confirm),
        '\x1B' => Some(GameEvent::Cancel), // ESC
        '\x7F' => Some(GameEvent::Back),   // Backspace
        // In-game actions
        'i' | 'I' => Some(GameEvent::OpenInventory),
        's' | 'S' => Some(GameEvent::OpenSpellbook),
        'n' | 'N' => Some(GameEvent::OpenJournal),
        'm' | 'M' => Some(GameEvent::OpenMap),
        'p' | 'P' => Some(GameEvent::SaveGame),
        'o' | 'O' => Some(GameEvent::LoadGame),
        'a' | 'A' => Some(GameEvent::Attack),
        '.' => Some(GameEvent::Wait),
        '?' => Some(GameEvent::OpenHelp),
        'b' | 'B' => Some(GameEvent::ToggleSound),
        'q' | 'Q' => Some(GameEvent::Quit),
        // Choice keys 1-9
        '1'..='9' => {
            let n = c.to_digit(10).unwrap();
            Some(GameEvent::Choice(n as u8))
        }
        _ => None,
    }
}
