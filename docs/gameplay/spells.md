# Spells

## Spell Slots

Tracked in `src/game/character/class.rs` as:

```rust
pub struct SpellSlots {
    pub max:  [u8; 9],   // index 0 = level 1 slots
    pub used: [u8; 9],
}
```

Recovering slots: Short rest recovers Warlock slots; Long rest recovers all
slots for all other classes.

---

## Spell TOML Schema (`assets/spells/<id>.toml`)

```toml
id              = "fireball"
name            = "Fireball"
level           = 3
school          = "evocation"
casting_time    = "action"       # action | bonus_action | reaction | ritual
range           = "150ft"
components      = ["V", "S", "M"]
duration        = "instantaneous"
concentration   = false
description     = "A bright streak flashes from your pointing finger..."

[effect]
type            = "damage"       # damage | heal | condition | summon | utility
damage          = "8d6"
damage_type     = "fire"
save            = "dexterity"    # ability for saving throw; null if attack roll
save_dc_source  = "spell"        # spell | fixed
aoe             = { shape = "sphere", radius = 20 }
upcast_bonus    = "1d6"          # additional damage per slot level above base
```

---

## Casting a Spell

1. Check class has the spell prepared/known.
2. Check enough spell slots at required level (cantrips cost no slots).
3. Expend one slot of chosen level.
4. Resolve effect per `[effect]` block:
   - **Damage**: targets make saving throw (if `save` set) or caster makes
     attack roll; apply damage with resistance/vulnerability.
   - **Heal**: restore HP to target.
   - **Condition**: apply condition (see [character.md](character.md)) with
     save if specified.

`src/game/combat/spells.rs`

---

## Concentration

If `concentration = true`, casting this spell immediately ends any other
concentration spell active on the caster. Tracked on `Character` as
`concentrating_on: Option<SpellId>`.

Taking damage while concentrating: Constitution saving throw,
DC = max(10, damage_taken / 2). Failure → concentration drops.

---

## Spellbook UI

`src/ui/screens/spellbook.rs`:
- Lists known/prepared spells grouped by level
- Shows spell details on selection
- Shows current slot counts
- Allows casting from exploration mode (healing, utility)
