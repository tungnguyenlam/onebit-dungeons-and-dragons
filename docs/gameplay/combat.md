# Combat

## 5e Action Economy

Each combatant gets per turn:
- **1 Action** — Attack, Cast a Spell, Dash, Disengage, Dodge, Help, Hide, Use Object
- **1 Bonus Action** — class features, certain spells, off-hand attack
- **1 Reaction** — opportunity attack, Shield spell, Counterspell (resets at start of your next turn)
- **Movement** — speed in tiles (tracked in `economy.rs`)

`src/game/combat/economy.rs` stores these as booleans on `CombatantState`:

```rust
pub struct ActionSlots {
    pub action: bool,
    pub bonus_action: bool,
    pub reaction: bool,
    pub movement_remaining: u32,
}
```

Slots reset at the start of the combatant's turn.

---

## Initiative

1. All combatants roll `1d20 + dex_modifier` at combat start.
2. Ties: player beats monsters; monster ties broken by index.
3. Result: `Vec<CombatantId>` sorted descending — the turn queue.
4. `src/game/combat/initiative.rs`

---

## Attack Roll

```
1d20 + attack_bonus >= target_AC  →  hit
```

`attack_bonus = ability_modifier + proficiency_bonus (if proficient)`

- Natural 20 → critical hit: roll damage dice twice.
- Natural 1 → automatic miss.

Weapon type determines ability modifier used:
- Melee weapon: STR (or DEX if Finesse property)
- Ranged weapon: DEX
- Spell attack: Spellcasting ability modifier

See → [dice.md](dice.md) for advantage/disadvantage rules.

---

## Damage Roll

```
damage_dice (DiceExpr) + ability_modifier
```

Damage type is declared by the weapon/spell (`slashing`, `fire`, `psychic`, etc.).
Resistance halves damage; vulnerability doubles it.

See → [items.md](items.md) for weapon DiceExprs, [spells.md](spells.md) for
spell damage.

---

## Conditions

Applied via `game/character/conditions.rs`. Stored as a `HashSet<Condition>`
on each entity. Evaluated at start/end of turn and on roll events.

Key conditions: Blinded, Charmed, Deafened, Exhaustion (levels 1-6),
Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified,
Poisoned, Prone, Restrained, Stunned, Unconscious.

See → [character.md](character.md) for full condition list.

---

## Combat Turn Flow

```
Start of turn:
  1. Reset ActionSlots
  2. Tick down conditions (reduce duration by 1)
  3. Apply start-of-turn condition effects (e.g. Regeneration, Poison damage)

During turn (player input or AI):
  4. Movement (subtract from movement_remaining)
  5. Action (sets action = false)
  6. Bonus action (sets bonus_action = false)
  7. Reactions happen in response to triggers, not turn order

End of turn:
  8. Apply end-of-turn effects
  9. Check for death (HP <= 0 → Unconscious + death saving throws)
 10. Advance turn queue
```

---

## Death & Dying

HP reaches 0 → `Unconscious` condition applied. On each of the creature's
turns it makes a death saving throw (`1d20`, DC 10):
- 3 successes → stabilised (regain 1 HP)
- 3 failures → dead
- Natural 20 → regain 1 HP immediately
- Natural 1 → counts as 2 failures

For monsters: reaching 0 HP is instant death.

---

## Combat UI

`src/ui/screens/combat.rs` — shows:
- Turn order banner (top)
- Battlefield grid (main area, uses region room tiles)
- Combatant stat bars (HUD)
- Action menu (numbered list) when it is the player's turn
- Log panel (bottom) streaming attack/damage/condition messages

See → [architecture/ui-layer.md](../architecture/ui-layer.md)
