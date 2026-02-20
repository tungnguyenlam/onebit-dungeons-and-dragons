# OneBit Dungeons & Dragons

> **If you are an AI agent or automated tool — read [`docs/AGENT.md`](docs/AGENT.md) first.**
> It tells you exactly where work stopped, what to do next, and which docs to read. Do not touch any code before completing all five steps in that file.

---

A terminal-based (TUI), open-world Dungeon & Dragons game written in **Rust**, following the **D&D 5e SRD** ruleset. The world is split into self-contained hand-crafted regions; story is delivered via NPC dialog, a player journal, and environmental lore.

**Tech stack:** Rust 2021 · Ratatui 0.29 · Crossterm · Serde + TOML · Rand · Clap

---

## Running the Game Locally

```bash
# Build/check
cargo check

# TUI mode (default renderer)
cargo run

# Explicit mode flag (same as above)
cargo run -- --mode tui

# GUI mode (experimental/stub)
cargo run --features gui -- --mode gui

# Run tests
cargo test
```

### TUI Controls

- `q` / `Ctrl-C`: quit
- `h j k l` or arrow keys: move cursor/navigation
- `a`: attack
- `.`: wait / advance
- `i`: inventory
- `s`: spellbook
- `n`: journal
- `m`: world map
- `1`..`9`: choice/select action
- `Esc` / `Backspace`: back/cancel

---

## Project Layout

```
onebit-dungeons-and-dragons/
│
├── Cargo.toml              ← dependencies + feature flags (tui / gui)
├── README.md               ← you are here
│
├── assets/                 ← all hand-crafted game content (TOML files)
│   ├── classes/            ← one .toml per playable class
│   ├── races/              ← one .toml per playable race
│   ├── monsters/           ← monster stat blocks
│   ├── items/              ← weapons, armor, consumables
│   ├── spells/             ← spell definitions
│   ├── quests/
│   │   ├── main/           ← main-story quest files
│   │   └── side/           ← side quest files
│   ├── lore/               ← lore / inspect text entries
│   └── regions/
│       └── <region-slug>/  ← one folder per region
│           ├── region.toml
│           ├── rooms/
│           ├── npcs/
│           └── dialog/
│
├── src/                    ← Rust source code
│   ├── main.rs             ← CLI flag parsing (--mode tui|gui), main loop
│   ├── app.rs              ← AppState enum, App struct, event dispatch
│   ├── renderer.rs         ← GameRenderer trait + GameEvent enum
│   │
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── tui/            ← Ratatui / Crossterm renderer [feature = "tui"]
│   │   │   ├── mod.rs      ← TuiRenderer: impl GameRenderer
│   │   │   └── screens/    ← combat/dialog/journal/inventory/spellbook
│   │   └── gui/            ← egui / eframe renderer [feature = "gui"]
│   │       └── mod.rs      ← GuiRenderer / GuiApp (stubbed — TUI first)
│   │
│   ├── game/               ← pure game logic — zero UI imports
│   │   ├── mod.rs
│   │   ├── dice/
│   │   │   ├── mod.rs      ← DiceExpr struct + roll / advantage / disadvantage
│   │   │   └── parser.rs   ← parse "2d6+3" strings
│   │   ├── character/
│   │   │   ├── mod.rs
│   │   │   ├── stats.rs    ← AbilityScores, Character, HP, AC
│   │   │   ├── skills.rs   ← Skill enum, SkillSet (proficiency / expertise)
│   │   │   ├── conditions.rs ← Condition enum (Poisoned, Stunned, …)
│   │   │   └── progression.rs ← proficiency bonus, XP thresholds, level-up HP
│   │   ├── items/
│   │   │   ├── mod.rs
│   │   │   ├── inventory.rs ← Inventory, ItemInstance
│   │   │   ├── equipment.rs ← EquipmentSlots
│   │   │   └── armor.rs    ← AC calculation by armor type
│   │   ├── world/          ← region, room, tile map, FOV
│   │   ├── combat/         ← initiative, attack, action economy, spell effects
│   │   ├── story/          ← WorldState, quest machine, dialog, journal, events
│   │   ├── npc/            ← (planned M5) monster AI
│   │   └── save/           ← (planned M7) save / load serialization
│   │
│   └── data/               ← TOML deserialization layer — no game logic
│       ├── mod.rs
│       ├── types.rs        ← serde structs for every TOML asset type
│       └── loader.rs       ← load<T>(), load_region(), load_global_assets()
│
└── docs/                   ← all design, architecture, and task docs
    ├── AGENT.md            ← AI agent entry point — READ THIS FIRST
    ├── architecture/
    │   ├── overview.md     ← module boundary rules, full source tree
    │   ├── game-loop.md    ← tick loop, event flow
    │   ├── ui-layer.md     ← screen inventory, layout convention
    │   ├── renderer.md     ← TUI vs GUI abstraction, GameRenderer trait
    │   └── data-pipeline.md ← TOML → typed structs → game modules
    ├── gameplay/           ← D&D 5e rules as implemented in this game
    │   ├── character.md · combat.md · dice.md · items.md
    │   ├── spells.md · world.md · story.md · dialog.md
    │   ├── journal.md · npc-ai.md · overview.md
    ├── content/            ← hand-crafted content schemas & lists
    │   ├── classes.md · races.md · monsters.md · spells-list.md
    │   ├── items-list.md · quests.md · lore.md · map-format.md
    │   └── regions/index.md
    ├── decisions/          ← Architecture Decision Records (ADRs)
    └── tasks/
        ├── backlog.md      ← ordered milestone task list
        ├── current-sprint.md ← active task + last-session handoff
        └── done.md         ← completed tasks
```

---

## Architecture in One Sentence

`src/game/` is pure logic; `src/ui/` is pure rendering; `src/app.rs` wires them together via a `GameEvent` enum — neither game nor UI ever imports the other.

For the detailed design see [docs/architecture/overview.md](docs/architecture/overview.md).

---

## Development Status

| Milestone | Status |
|---|---|
| M0 — Crate bootstrap & renderer abstraction | ✅ Done |
| M1 — Core systems (dice, character, data layer) | ✅ Done |
| M2 — Combat | ✅ Done |
| M3 — Story & Dialog | ✅ Done |
| M4 — Items & Spells | ✅ Done |
| M5 — NPC & Factions | 🔧 Next |
| M6 — First Region (Valley of Ash) | ⬜ Planned |
| M7 — Polish & Save/Load | ⬜ Planned |

---

## Contributing / Continuing Development

See [docs/AGENT.md](docs/AGENT.md) — the workflow applies equally to humans and AI agents.
