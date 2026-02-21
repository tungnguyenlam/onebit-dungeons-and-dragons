# ADR-001 — Ratatui State Machine

**Date:** 2026-02-20  
**Status:** Accepted

## Decision

`AppState` is an enum owned by `App` in `src/app/mod.rs`. Each variant holds only
the context needed for that screen (e.g. `Combat(CombatContext)`,
`Dialog(DialogContext)`). The UI layer matches on `AppState` to decide which
screen to render.

## Rationale

- Exhaustive match forces the compiler to ensure every state is handled.
- Screen-specific context (e.g. which dialog node is active) is stored in the
  variant, not in a global mutable field, preventing stale-state bugs.
- Avoids complex "screen stack" abstractions for a game with a small, known
  set of screens.

## Rejected Alternatives

- **Screen stack (`Vec<Box<dyn Screen>>`)**: more flexible but adds allocations
  and dynamic dispatch overhead with no benefit given the fixed screen set.
- **Separate UI state struct**: would duplicate state management logic and make
  `AppState` transitions harder to audit.

## Consequences

- Adding a new screen requires adding an `AppState` variant + handling it in
  `App::render()` and `App::handle_event()`.
- Transitions are explicit calls to `App::transition(AppState)` — no implicit
  navigation.
