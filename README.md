# OneBit Dungeons & Dragons

> **If you are an AI agent or automated tool — read [`AGENT.md`](AGENT.md) first.**
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

# Manual interactive TUI playtest (requires TTY terminal)
scripts/agent_tui_smoke.sh --interactive

# Asset validation (rooms/quests/dialog links + reachability)
cargo run -- --validate-assets
scripts/validate_assets.sh
```

---

## Step-Through Testing (for Agents)

This is the **recommended way for agents to test the game** without needing a terminal/TTY. Each keypress is processed one at a time, and the game state is dumped as text after each action.

Preferred tool name: `visual_check` script (`scripts/visual_check.py`).

### Quick Start

```bash
# Dump initial game state (main menu)
scripts/runtest.sh

# Press a key, see the result
scripts/runtest.sh j           # move down / vim-style down
scripts/runtest.sh k           # move up
scripts/runtest.sh h           # move left
scripts/runtest.sh l           # move right
scripts/runtest.sh $'\r'       # Enter / confirm
scripts/runtest.sh a           # attack
scripts/runtest.sh i           # inventory
scripts/runtest.sh n           # journal
scripts/runtest.sh m           # map
scripts/runtest.sh ?           # help
scripts/runtest.sh p           # save game
scripts/runtest.sh q           # quit
```

### How It Works

1. **Run with a key**: `scripts/runtest.sh j`
2. **Game processes that ONE keypress**
3. **Game state is dumped as text** showing:
   - Current state (menu, combat, etc.)
   - Player stats (HP, XP, gold, level)
   - Inventory contents
   - Current room layout with player position (`@`)
   - NPCs and triggers in the room

### Why This is Useful for Agents

- **No TTY required** - works in any environment
- **See exactly what happens** after each input
- **Verify bug fixes** by checking state changes
- **Test gameplay mechanics** step by step
- **Parse output easily** - plain text format

### Key Mappings

| Key | Action |
|-----|--------|
| `j` / `k` | Move down / up |
| `h` / `l` | Move left / right |
| `Enter` | Confirm / select |
| `Esc` | Cancel / back |
| `i` | Inventory |
| `s` | Spellbook |
| `n` | Journal |
| `m` | World map |
| `?` | Help / legend |
| `a` | Attack |
| `.` | Wait |
| `p` | Save game |
| `o` | Load game |
| `q` | Quit |
| `1-9` | Dialog choices |

### Example Output

```
========================================
GAME STATE
========================================

--- App State ---
State: WorldMap
Turn: 5
Current Room: ash_gate
Player Position: (3, 2)

--- Player ---
Name: Theron
Level: 1
XP: 0
HP: 24/24
Gold: 10

--- Room Grid ---
##############
#..@..!......#
##############

========================================
```

For more details, see [`docs/testing/step-through-testing.md`](docs/testing/step-through-testing.md).

### Visual Check CLI (`scripts/visual_check.py`)

Use this name when referring to the scenario-based headless snapshot runner.

```bash
# List scenarios
python3 scripts/visual_check.py -l

# Run a scenario, save compact final artifact (default)
python3 scripts/visual_check.py --scenario enter_world

# Show output without writing artifact files
python3 scripts/visual_check.py --scenario enter_world --artifact none --show

# Capture full step-by-step artifact history
python3 scripts/visual_check.py --scenario enter_world --verbose-steps --artifact full --history
```

Note: this CLI is not independent from game logic. It drives the actual game binary in text mode, so output changes whenever gameplay or rendering logic changes.

---

### TUI Controls

- `q` / `Ctrl-C`: quit
- `h j k l` or arrow keys: move cursor/navigation
- `a`: attack
- `.`: wait / advance
- `i`: inventory
- `s`: spellbook
- `n`: journal
- `m`: world map
- `?`: toggle help/legend
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
├── AGENT.md            ← AI agent entry point — READ THIS FIRST
└── docs/                   ← all design, architecture, and task docs
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

Milestone tracking is maintained in:
- [docs/tasks/current-sprint.md](docs/tasks/current-sprint.md) for active/in-progress work
- [docs/tasks/backlog.md](docs/tasks/backlog.md) for the milestone index and upcoming work
- [docs/tasks/done.md](docs/tasks/done.md) for completion records

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

See [AGENT.md](AGENT.md) — the workflow applies equally to humans and AI agents.
For doc synchronization rules, also see [docs/DOCS_MAP.md](docs/DOCS_MAP.md).
For automated non-interactive TUI validation, see [docs/testing/tui-agent-smoke.md](docs/testing/tui-agent-smoke.md).
Release notes: [docs/releases/v0.1.0-internal.md](docs/releases/v0.1.0-internal.md).
