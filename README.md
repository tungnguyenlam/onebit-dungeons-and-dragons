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

# Standard agent verification entry point
scripts/agent_verify.sh

# Full release-readiness check
scripts/release_check.sh

# Agent-run automated TUI smoke flow
scripts/agent_tui_smoke.sh
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
- `p`: save game (`saves/slot1.toml`)
- `o`: load game (`saves/slot1.toml`)
- `b`: toggle bell sound
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
│   │   │   └── screens/    ← main_menu/world_map/combat/dialog/journal/inventory/spellbook/character_creation/game_over
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
│   │   └── save/           ← save / load serialization helpers
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
| M5 — NPC & Factions | ✅ Done |
| M6 — First Region (Valley of Ash) | ✅ Done |
| M7 — Polish & Save/Load | ✅ Done |
| M8 — Stability & Engineering Debt | ✅ Done |
| M9 — Core RPG Depth | ✅ Done |
| M10 — Content Production Pipeline | ✅ Done |
| M11 — NPC/Faction Simulation 2.0 | ✅ Done |
| M12 — UX & Presentation Polish 2.0 | ✅ Done |
| M13 — Release Readiness | ✅ Done |

---

## Terminal Support Matrix

Runtime visual tier is auto-detected in TUI startup:

| Tier | Capability | Policy |
|---|---|---|
| `T0` | ASCII-only / no color | ASCII icons + plain styling fallback |
| `T1` | UTF-8 glyph support | Unicode icon fallback + minimal color |
| `T2` | 256-color terminal | Semantic palette tokens enabled |
| `T3` | Truecolor terminal | Full semantic palette enabled |

Color/visual tokens are centralized in `src/ui/tui/theme.rs`.

---

## TUI Screenshots

### Main Menu

```text
┌ Main Menu ────────────────────────────────────────┐
│ OneBit Dungeons & Dragons                         │
├ Options ──────────────────────────────────────────┤
│ > New Game                                        │
│   Continue                                        │
│   Load Save                                       │
│   Quit (press q)                                  │
└────────────────────────────────────────────────────┘
```

### World Map

```text
┌ World ────────────────────────────────────────────┐
│ Region: Valley of Ash (valley-of-ash)            │
│ Room: ash_gate                                   │
│ Player: Theron at (3, 2)                         │
├ Map ──────────────────────────────────────────────┤
│ ##########                                        │
│ #........#                                        │
│ #..@..!..#                                        │
│ #....+...#                                        │
│ ##########                                        │
└────────────────────────────────────────────────────┘
```

---

## Contributing / Continuing Development

See [docs/AGENT.md](docs/AGENT.md) — the workflow applies equally to humans and AI agents.
For doc synchronization rules, also see [docs/DOCS_MAP.md](docs/DOCS_MAP.md).
For automated non-interactive TUI validation, see [docs/testing/tui-agent-smoke.md](docs/testing/tui-agent-smoke.md).
Release notes: [docs/releases/v0.1.0-internal.md](docs/releases/v0.1.0-internal.md).
