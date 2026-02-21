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

pub fn run_text_mode(mut app: App, step: bool, key: &str) -> anyhow::Result<()> {
    use ratatui::{backend::TestBackend, Terminal};
    
    // Attempt to load existing test state to persist between step runs
    let _ = app.load_from_default_path();

    // 1. Process the input key if we are stepping
    if step && !key.is_empty() {
        let key_char = key.chars().next().unwrap_or(' ');
        if let Some(event) = char_to_game_event(key_char) {
            let _ = app.handle_event(event);
        }
    }

    // Save the state for the next runtest.sh invocation
    let _ = app.save_to_default_path();

    // 2. Set up a headless 88x24 TUI backend (matches your grid size)
    let backend = TestBackend::new(88, 24);
    let mut terminal = Terminal::new(backend)?;

    // 3. Render the actual TUI to our headless buffer
    terminal.draw(|f| {
        match &app.state {
            app::AppState::MainMenu => ui::tui::screens::main_menu::render(f, &app),
            app::AppState::CharacterCreation => ui::tui::screens::character_creation::render(f, &app),
            app::AppState::WorldMap => ui::tui::screens::world_map::render(f, &app),
            app::AppState::Combat(_) => ui::tui::screens::combat::render(f, &app),
            app::AppState::Dialog(_) => ui::tui::screens::dialog::render(f, &app),
            app::AppState::Journal => ui::tui::screens::journal::render(f, &app),
            app::AppState::Inventory => ui::tui::screens::inventory::render(f, &app),
            app::AppState::Spellbook => ui::tui::screens::spellbook::render(f, &app),
            app::AppState::Settings => ui::tui::screens::settings::render(f, &app),
            app::AppState::GameOver => ui::tui::screens::game_over::render(f, &app),
        }
    })?;

    // 4. Print the buffer row by row to standard output
    let buffer = terminal.backend().buffer();
    for y in 0..buffer.area.height {
        let mut row = String::with_capacity(buffer.area.width as usize);
        for x in 0..buffer.area.width {
            row.push_str(buffer.cell((x, y)).unwrap().symbol());
        }
        println!("{}", row);
    }
    
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
