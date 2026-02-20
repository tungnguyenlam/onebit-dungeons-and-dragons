# Dice

## DiceExpr Format

All dice rolls in TOML files and game code use the `DiceExpr` string format:

```
<count>d<sides>[+<modifier>][-<modifier>]
```

Examples: `1d20`, `2d6+3`, `1d8-1`, `4d6`

---

## Implementation: `src/game/dice/`

### `mod.rs`

```rust
pub struct DiceExpr {
    pub count: u32,
    pub sides: u32,
    pub modifier: i32,  // positive or negative
}

impl DiceExpr {
    pub fn roll(&self) -> i32 { ... }
    pub fn roll_advantage(&self) -> i32 { /* roll twice, take higher */ }
    pub fn roll_disadvantage(&self) -> i32 { /* roll twice, take lower */ }
}
```

### `parser.rs`

`fn parse(s: &str) -> Result<DiceExpr>` — parse a `DiceExpr` from string.
Used by the TOML deserialiser via a custom `serde` visitor so asset files can
write `damage = "2d6+3"` and get a `DiceExpr` directly.

---

## Ability Check

```
d20 + ability_modifier + proficiency_bonus (if proficient) >= DC
```

- Advantage: roll `1d20` twice, take higher.
- Disadvantage: roll `1d20` twice, take lower.
- Natural 20: always succeeds (attack rolls only; not ability checks by default).
- Natural 1: always fails (attack rolls only).

---

## Saving Throw

Same formula as ability check but uses the save's ability modifier.
Proficiency bonus applies if the character has proficiency in that save.

```
d20 + save_modifier >= DC
```

DC source depends on context:
- Spell save DC = `8 + spellcasting_modifier + proficiency_bonus`
- Trap DC = defined in room TOML

---

## Dice Roll UI

After any significant roll, the `widgets/dice_roll.rs` widget fires a
brief animated "rolling dice" overlay that shows the dice type, raw result,
modifier, and total. Duration: 1.5 s, then auto-dismisses.

See → [architecture/ui-layer.md](../architecture/ui-layer.md)
