# Code Map: Concept to File

This guide helps agents and developers locate specific logic quickly.

## Game Systems

| Concept | File / Directory |
|---|---|
| **Damage Calculation** | `src/game/combat/attack.rs` |
| **HP & Death Mechanics** | `src/game/combat/combat.rs` (CombatantState) |
| **XP & Leveling Charts** | `src/game/character/progression.rs` |
| **Granting XP Logic** | `src/app/mod.rs` (`grant_player_xp`) |
| **AC Formula** | `src/game/items/armor.rs` |
| **To-Hit Formula** | `src/game/combat/attack.rs` (`roll_attack`) |
| **Saving Throws** | `src/game/combat/attack.rs` (`roll_saving_throw`) |
| **Condition Effects** | `src/game/character/conditions.rs` |
| **Spell Resolution** | `src/game/combat/spells.rs` (logic) + `src/app/mod.rs` (casting) |
| **Inventory & Equipment** | `src/game/items/` |
| **TOML Deserialization** | `src/data/types.rs` |
| **Asset Loading** | `src/data/loader.rs` |

## State Management

| State | File |
|---|---|
| **Global Flags (WorldState)** | `src/game/story/world_state.rs` |
| **Active Quest Tracking** | `src/game/story/quest.rs` |
| **UI Screen Definitions** | `src/app/state.rs` |
| **Event Dispatcher** | `src/app/handlers.rs` |

## Common Tasks

- **Add a new Item Type**:
    1. Update `ItemType` enum in `src/data/types.rs`.
    2. Add parsing logic in `src/data/loader.rs` if needed.
    3. Add a sample in `src/app/samples.rs`.
- **Add a new Monster Action**:
    1. Update `MonsterAction` struct in `src/data/types.rs`.
    2. Add the action to `combatant_from_monster` in `src/app/samples.rs`.
- **Add a new UI Screen**:
    1. Add a variant to `AppState` in `src/app/state.rs`.
    2. Add a handler in `src/app/handlers.rs`.
    3. Add a renderer in `src/ui/tui/screens/`.
