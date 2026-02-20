# Story System

## Architecture

```
WorldState (flag store)
    ↑ read/write
    │
    ├── Quest machine    (quest.rs)    — stage advancement, objective tracking
    ├── Dialog evaluator (dialog.rs)   — condition checks on dialog nodes
    ├── Journal          (journal.rs)  — appends entries on triggers
    └── Emergent events  (events.rs)   — world events fired by WorldState thresholds
```

`WorldState` is the single shared source of truth. No story module directly
mutates character stats — it fires **events** that `app.rs` routes to the
appropriate game subsystem.

---

## WorldState (`src/game/story/world_state.rs`)

A flat key/value store:

```rust
pub struct WorldState {
    pub flags:   HashMap<String, bool>,   // e.g. "killed_bandit_lord" = true
    pub counters: HashMap<String, i32>,   // e.g. "faction_guild_rep" = 12
}
```

Helper predicates used in TOML condition strings:

| Syntax | Meaning |
|---|---|
| `flag:key` | `flags["key"] == true` |
| `not flag:key` | `flags["key"] != true` |
| `counter:key >= N` | `counters["key"] >= N` |
| `counter:key < N` | `counters["key"] < N` |

Conditions in TOML are single-line strings; `&&` and `\|\|` are supported.

---

## Quest Stage Machine (`src/game/story/quest.rs`)

Each quest is a TOML file in `assets/quests/`. A quest has stages; the player
is always at one stage per quest. Advancing a stage requires a condition over
`WorldState`.

```toml
id    = "bandit-king"
name  = "The Bandit King"
type  = "main"

[[stages]]
id        = "start"
label     = "Investigate the burned village"
condition = ""         # always active when quest is accepted
on_enter  = []         # list of flag/counter mutations
next      = [{ condition = "flag:found_bandits", stage = "track" }]
journal_entry = "The village of Mirefall lies in ashes..."

[[stages]]
id        = "track"
label     = "Track the bandits to their lair"
condition = "flag:found_bandits"
next      = [{ condition = "flag:entered_bandit_lair", stage = "confront" }]
journal_entry = "Tracks lead north, toward the Ironhold Mines."

[[stages]]
id        = "confront"
label     = "Defeat or confront the Bandit King"
condition = "flag:entered_bandit_lair"
next      = [
  { condition = "flag:killed_bandit_lord",    stage = "done_kill"    },
  { condition = "flag:persuaded_bandit_lord", stage = "done_persuade" },
]
journal_entry = "The Bandit King awaits in the throne room."
```

---

## Emergent World Events (`src/game/story/events.rs`)

Event triggers are registered at startup by reading all quest and faction
definitions. Each trigger is:

```rust
pub struct EventTrigger {
    pub condition: ConditionExpr,    // evaluated against WorldState
    pub event:     WorldEvent,       // what happens
    pub once:      bool,             // fire only once?
}
```

Examples of `WorldEvent`:
- `SpawnEncounter { region, room, encounter_id }` — ambush after rep drops
- `UnlockRegion { region_slug }` — new area opens after story milestone
- `ModifyShopInventory { shop_id, add_items, remove_items }`
- `AddJournalEntry { entry_id }`

Triggered on `Event::Tick` in the game loop.

---

## Quest Acceptance

Quests are offered via dialog choices or world triggers. Accepting a quest:
1. Sets `flag:quest_<id>_active = true` in WorldState.
2. Sets the quest's current stage to `"start"`.
3. Appends the first journal entry to the journal.

See → [dialog.md](dialog.md), [journal.md](journal.md)

---

## Quest Diagnostics (M22) (`QuestLog::blocked_quests`)

When a quest is *active* but no transitions are satisfiable, it is **blocked**.

```rust
// Check all active quests for stuck states
let blocked: Vec<QuestBlockedDiag> = quest_log.blocked_quests(&world_state);

// Emit recovery hint entries into the journal
let count = quest_log.emit_blocked_hints(&world_state, &mut journal, turn);
```

`QuestBlockedDiag` fields:
- `quest_id` — which quest is stuck
- `stage_id` — which stage it's blocked on
- `reason: BlockedReason` — `NoSatisfiedTransition | MissingStage | MissingDef`

Hints are written as `Category::System` journal entries visible in the **System** tab.  
A stage with *no* `next` entries is a terminal stage, not stuck — it is not reported.
