# Writing & Running Tests

## Unit Tests
Unit tests live in the same file as the logic (`mod.rs` or specific sub-modules).

### Seeded Attack Tests
Combat logic should be tested using `roll_attack_with_seed` for determinism.
```rust
#[test]
fn test_seeded_hit() {
    let atk = AttackProfile { ... };
    let def = DefenseProfile { ... };
    let out = roll_attack_with_seed(&atk, &def, 42); // Always results in same d20
    assert_eq!(out.hit_type, HitType::Hit);
}
```

## Integration Tests
Integration tests live in `src/app/tests.rs`. These usually test the `App` state machine via `GameEvent` dispatch.

### Mocking the App
Use `App::new()` which loads `samples.rs` fallbacks if real assets are missing.
```rust
#[test]
fn test_feature() {
    let mut app = App::new();
    app.transition(AppState::WorldMap);
    app.handle_event(GameEvent::MoveUp).unwrap();
    assert_eq!(app.player_pos.1, 0);
}
```

## Smoke & Soak Tests
For UI-level verification, use the agent smoke scripts:
- **Standard Check**: `scripts/agent_verify.sh --with-smoke`
- **Soak (stress test)**: `scripts/agent_tui_smoke.sh --soak --minutes 5`

## Validation
To verify all TOML assets have valid cross-references (NPCs exist, Dialog files are linked, etc.):
- `scripts/validate_assets.sh` (or `cargo run -- --validate-assets`)
