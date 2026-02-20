# Game Loop

## Overview

The main loop runs on a single thread. Crossterm delivers raw keyboard events;
a 250 ms tick fires repeatedly to advance time-based state (animations,
cooldowns, emergent event checks).

---

## AppState Enum

`src/app.rs` owns an `AppState` enum that determines which UI screen is active
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

`src/events.rs`:

```rust
pub enum Event {
    Key(KeyEvent),
    Tick,
    Quit,
}
```

The event loop (in `main.rs`) reads from a crossterm event stream, converts to
`Event`, and sends to `App::handle_event(&mut self, event: Event)`.

---

## Tick Loop

```
main loop:
  select! {
    key event -> App::handle_event(Event::Key)
    tick (250ms) -> App::handle_event(Event::Tick)
  }
  App::render(frame) via Terminal::draw()
```

On `Event::Tick`:
1. Advance any active combat animations
2. Check WorldState emergent event triggers (`game/story/events.rs`)
3. Age any temporary conditions on all entities

---

## Render Path

`App::render()` calls one of the screen render functions in `src/ui/screens/`
based on current `AppState`. Screen functions receive a shared reference to
`App` (read-only game state) and a mutable Ratatui `Frame`.

See → [ui-layer.md](ui-layer.md)
