# Items & Equipment

## Inventory

`src/game/items/inventory.rs` stores a `Vec<ItemInstance>` (ordered list).
Max carry weight is tracked loosely (item count limit, not detailed encumbrance
by default — can revisit).

Each `ItemInstance` has:
- `item_id: String` — resolves to global item def in `assets/items/`
- `quantity: u32` — for stackable items (potions, arrows, gold)
- `equipped: bool`

---

## Equipment Slots

```rust
pub struct EquipmentSlots {
    pub main_hand:   Option<ItemId>,  // weapon
    pub off_hand:    Option<ItemId>,  // weapon or shield
    pub armor:       Option<ItemId>,
    pub helmet:      Option<ItemId>,
    pub boots:       Option<ItemId>,
    pub ring_1:      Option<ItemId>,
    pub ring_2:      Option<ItemId>,
    pub amulet:      Option<ItemId>,
}
```

`src/game/items/equipment.rs`

---

## Item TOML Schema (`assets/items/<id>.toml`)

```toml
id          = "longsword"
name        = "Longsword"
type        = "weapon"      # weapon | armor | consumable | misc | quest
weight      = 3
value_gp    = 15
description = "A standard steel longsword."

[weapon]
damage      = "1d8"
damage_type = "slashing"
properties  = ["versatile"]   # finesse | versatile | thrown | heavy | light | reach | ranged | ammunition | loading | two-handed
versatile_damage = "1d10"
range       = null            # { normal = 30, long = 120 } for ranged

[armor]  # omit if not armor
base_ac     = 16
type        = "heavy"         # light | medium | heavy | shield
stealth_disadvantage = true
```

---

## Armor Class Calculation

- No armor: `AC = 10 + DEX_modifier`
- Light armor: `AC = base_ac + DEX_modifier`
- Medium armor: `AC = base_ac + min(DEX_modifier, 2)`
- Heavy armor: `AC = base_ac` (no DEX)
- Shield: `+2 AC` (stacks with armor, occupies off-hand)

`src/game/items/armor.rs`

---

## Consumables

Potions, scrolls, ammunition. `use_item()` in `src/game/items/inventory.rs`
applies the item's mechanical effect (heal HP, cast a spell, etc.) and
decrements quantity (or removes if quantity reaches 0).

See → [gameplay/spells.md](spells.md) for scroll spell resolution.
