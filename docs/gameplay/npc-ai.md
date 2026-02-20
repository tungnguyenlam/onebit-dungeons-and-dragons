# NPC AI

## Overview

NPC/monster AI currently runs in `src/app.rs` enemy-turn handling, using
runtime metadata stored on `CombatantState` (`EnemyAiRole` plus optional ranged
and spell attack profiles).

Monster templates are loaded from `assets/monsters/*.toml` via
`src/data/loader.rs::load_monsters`, then converted into combatants for
encounters.

---

## Monster Stat Block (`assets/monsters/<id>.toml`)

```toml
id         = "goblin"
name       = "Goblin"
size       = "small"
type       = "humanoid"
alignment  = "neutral_evil"
ac         = 15
hp         = "2d6"
speed      = 30
xp         = 50
cr         = "0.25"

[stats]
str = 8
dex = 14
con = 10
int = 10
wis = 8
cha = 8

[saves]
# only list non-default saves
dex = 4

[[actions]]
name       = "Scimitar"
type       = "melee_attack"
bonus      = 4
reach      = 5
damage     = "1d6+2"
damage_type= "slashing"

[[actions]]
name       = "Shortbow"
type       = "ranged_attack"
bonus      = 4
range      = { normal = 80, long = 320 }
damage     = "1d6+2"
damage_type= "piercing"

[ai]
behaviour  = "skirmisher"   # skirmisher | brute | ranged | spellcaster | coward
flee_at_hp = 0.25           # fraction of max HP — flees if below this
```

---

## AI Behaviour Types (Current Runtime)

| Behaviour | Decision logic |
|---|---|
| `melee` | Uses base melee attack profile and nearest opposing target |
| `ranged` | Prefers ranged profile and prioritizes lowest-HP opposing target |
| `spellcaster` | Prefers spell profile (including on-hit condition if present) and lowest-HP target |

---

## Faction Reputation

NPCs belong to a faction (`faction_id` in their TOML). Faction reputation is a
signed integer in `WorldState`, e.g. `"faction_guild_rep" = 12`.

Thresholds (configurable per faction):
- `>= 10` → Friendly (better prices, dialog options, will assist in combat)
- `0–9` → Neutral
- `< 0` → Hostile (attack on sight if below `hostile_threshold`)

Quest outcomes, combat, and dialog choices all modify faction rep by firing
`WorldState` flag deltas. See → [gameplay/story.md](story.md)

---

## Dialog Outside Combat

NPCs with a `dialog/<id>.toml` file can be spoken to when the player enters
their trigger zone or presses interact. AI is irrelevant during dialog — the
dialog tree system takes over entirely.

See → [gameplay/dialog.md](dialog.md)
