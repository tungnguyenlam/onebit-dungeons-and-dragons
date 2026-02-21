# Game Loop

## Overview

The main loop runs on a single thread. Crossterm delivers raw keyboard events;
a 250 ms tick fires repeatedly to advance time-based state (animations,
cooldowns, emergent event checks).

---

## AppState Enum

`src/app/mod.rs` owns an `AppState` enum that determines which UI screen is active
and which game subsystems receive input:

```rust
pub enum AppState {
    MainMenu,
    CharacterCreation,
    WorldMap,          // exploring region
    Combat(CombatContext),
    Dialog(DialogContext),
    Journal,
    Inventory,
    Spellbook,
    GameOver,
}
```

Transitions happen via `App::transition(next: AppState)`. The UI layer reads
`AppState` to decide which screen to render. The event handler dispatches
input to the correct subsystem based on the same `AppState`.

---

## Event Enum

`src/renderer.rs` defines a renderer-agnostic `GameEvent` that both
renderers produce:

```rust
pub enum GameEvent {
    // Navigation
    MoveUp, MoveDown, MoveLeft, MoveRight,
    Confirm, Cancel, Back,
    // In-game actions
    OpenInventory, OpenSpellbook, OpenJournal, OpenMap,
    Attack, Wait,
    // Dialog choice (1–9)
    Choice(u8),
    // System
    Tick,
    Quit,
}
```

The TUI renderer maps raw crossterm `KeyEvent` values to `GameEvent`.
The GUI renderer maps egui keyboard events to `GameEvent`.
`App::handle_event(&mut self, event: GameEvent)` never sees raw key codes.

See → [renderer.md](renderer.md)

---

## Main Loop (Renderer-Agnostic)

`main.rs` selects the renderer from `--mode tui|gui`, then drives the same
loop regardless of which renderer is active:

```rust
fn run_loop(mut renderer: Box<dyn GameRenderer>, mut app: App)
    -> anyhow::Result<()>
{
    loop {
        renderer.render(&app)?;            // draw TUI frame or egui window
        match renderer.poll_event()? {     // block ≤250 ms
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
```

The TUI renderer's `poll_event` uses a crossterm event stream with a 250 ms
timeout; timeout returns `GameEvent::Tick`.
The GUI renderer's tick is driven by `eframe`'s repaint request.

On `Event::Tick`:
1. Advance any active combat animations
2. Check WorldState emergent event triggers (`game/story/events.rs`)
3. Age any temporary conditions on all entities

---

## Render Path

`GameRenderer::render(&mut self, app: &App)` is called each iteration.

- **TUI**: calls one of the screen render functions in `src/ui/tui/screens/`
  based on `app.state`. Each function receives a Ratatui `Frame`.
- **GUI**: `eframe` triggers `GuiRenderer::update()` on every repaint. The
  renderer reads `app.state` and dispatches to the matching egui screen.

Both paths receive only a **shared** reference to `App` — no mutations happen
during rendering.

See → [ui-layer.md](ui-layer.md) · [renderer.md](renderer.md)
