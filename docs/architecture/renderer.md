# Renderer Abstraction

## Goal

The game supports two front-ends that can be selected at launch:

| Mode | Flag | Library | Use case |
|---|---|---|---|
| **TUI** | `--mode tui` | Ratatui + Crossterm | Terminal, SSH, low-resource |
| **GUI** | `--mode gui` | egui + eframe | Desktop windowed app |

The game back-end (`src/game/`) has **zero knowledge** of which renderer is
active. `src/app/` is also renderer-agnostic; it only operates on
`AppState` and `GameEvent`.

---

## Cargo Features

```toml
[features]
default = ["tui"]
tui     = ["dep:ratatui", "dep:crossterm"]
gui     = ["dep:eframe", "dep:egui"]
```

Both features can be compiled in simultaneously. The runtime mode is chosen
via the `--mode` CLI flag. This allows a single shipped binary that supports
either front-end.

```bash
cargo run                                    # TUI (default feature)
cargo run --features gui -- --mode gui       # GUI window
cargo run --features gui -- --mode tui       # TUI, with GUI feature compiled in
cargo run --no-default-features \
          --features gui -- --mode gui       # GUI-only binary
```

---

## `GameRenderer` Trait

`src/renderer.rs` defines the single trait both renderers implement:

```rust
/// Implemented by TuiRenderer and GuiRenderer.
/// The game loop in main.rs calls these methods; it never touches
/// ratatui Frame or egui Context directly.
pub trait GameRenderer {
    /// Draw the current app state to the screen / window.
    fn render(&mut self, app: &App) -> anyhow::Result<()>;

    /// Block until the next input event or tick timeout, then return it.
    /// Returning `GameEvent::Quit` causes the main loop to exit.
    fn poll_event(&mut self) -> anyhow::Result<GameEvent>;

    /// Restore the environment (e.g. reset raw mode, drop window).
    fn teardown(self: Box<Self>) -> anyhow::Result<()>;
}
```

---

## Renderer-Agnostic Main Loop

`src/main.rs` selects the renderer based on the CLI flag, then hands off to
a shared loop:

```rust
fn run_loop(mut renderer: Box<dyn GameRenderer>, mut app: App)
    -> anyhow::Result<()>
{
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
```

---

## TUI Renderer — `src/ui/tui/`

Implemented behind `#[cfg(feature = "tui")]`.

| File | Responsibility |
|---|---|
| `mod.rs` | `TuiRenderer` struct, `impl GameRenderer` |
| `theme.rs` | Color/icon themes, capability tiers |
| `vfx.rs` | Visual effects (animations, particles) |
| `screens/` | One `render(f, app)` fn per `AppState` variant |
| `widgets/` | Reusable Ratatui widgets |

`TuiRenderer::poll_event` reads from a crossterm event stream with a 250 ms
timeout; timeout returns `GameEvent::Tick`.

`TuiRenderer::teardown` disables raw mode and restores the alternate screen.

---

## GUI Renderer — `src/ui/gui/`

Implemented behind `#[cfg(feature = "gui")]`.

Uses `eframe` as the window/event host and `egui` for all drawing.

| File | Responsibility |
|---|---|
| `mod.rs` | `GuiRenderer` struct, `impl GameRenderer`, `impl eframe::App` |
| `screens/` | One `draw(ui, app)` fn per `AppState` variant |
| `widgets/` | Reusable egui widgets (HP bar, dice pop-up, log panel) |

`eframe` owns its own event loop. `GuiRenderer` adapts it: egui keyboard
events are translated to `GameEvent` values before being stored in an
internal channel, so `poll_event` can return them through the same interface.

The GUI targets the same visual information as the TUI; it is **not** a richer
game — just a windowed rendering of the same `AppState`.

---

## `GameEvent` — Renderer-Agnostic Input

`src/renderer.rs` defines events in game terms, not raw key codes:

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

Each renderer maps its own raw events to `GameEvent`. If a key has no mapping
it is silently ignored. This means the game logic in `src/game/` and
`src/app/` never sees platform-specific key codes.

---

## Adding a Third Renderer

1. Add a new Cargo feature, e.g. `web = ["dep:…"]`.
2. Create `src/ui/web/mod.rs` and implement `GameRenderer`.
3. Add a new `LaunchMode::Web` variant in `src/main.rs` and a match arm in
   the CLI dispatch.

No changes are required to `src/app/` or any `src/game/` module.

---

## Decision Records

- [ADR-001 — Ratatui state design](../decisions/adr-001-ratatui-state.md)
- See also: [ui-layer.md](ui-layer.md) for screen inventory
