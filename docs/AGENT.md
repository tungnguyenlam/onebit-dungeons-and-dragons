# AGENT.md — Cold-Start Entry Point

> **Read this file first. It is the only file you need to orient yourself.
> All other docs are linked from here. Do not read files not linked from your
> current task.**

---

## How to Resume Development (read this every time)

Follow these steps **in order** before touching any code or content:

**Step 1 — Read the handoff block**  
Open [tasks/current-sprint.md](tasks/current-sprint.md) and read the
`## Last Session Handoff` block at the top. It tells you:
- What task was in progress
- Which files were being edited
- Exactly what was done and what is left
- Any blockers or open decisions

**Step 2 — Verify the code compiles**
```bash
cargo check 2>&1 | head -40
```
If it fails, check the handoff block — the previous agent may have noted a
known compile error as their stopping point. Fix it before continuing.

`cargo` warnings are currently **non-blocking**. Do not do broad warning-only
cleanup unless the active task explicitly requests it. Prioritize feature and
behavior work; warning cleanup is deferred.

---

## Testing the Game (Choose One)

### Option 1: Visual Step-Through Testing (Recommended)

This is the **primary way for agents to test gameplay** - it uses a dedicated tool to manage scenarios and snapshots:

```bash
# List available test scenarios
python3 scripts/visual_check.py -l

# Run a specific scenario and show output
python3 scripts/visual_check.py --scenario enter_world --show

# Run custom keys and save as a named snapshot
python3 scripts/visual_check.py "llll" --name moving_east --show

# Reset state and run keys
python3 scripts/visual_check.py "a" --reset --show
```

**Why this is great for agents:**
- **Scenarios**: Repeatable sequences defined in `tests/visual_scenarios.json`.
- **Snapshots**: Automatically saves the TUI to `test_outputs/` for comparison.
- **Persistence**: Handles the `save.toml` state management for you.
- **Convenience**: No TTY needed; text output is easy to review.

Guide: [testing/step-through-testing.md](testing/step-through-testing.md)

### Option 2: TUI Smoke Test (Automated)

For a non-interactive keyboard smoke check of the TUI:
```bash
scripts/agent_tui_smoke.sh
```
Guide: [testing/tui-agent-smoke.md](testing/tui-agent-smoke.md)

For manual interactive playtest:
```bash
scripts/agent_tui_smoke.sh --interactive
```
Note: this requires a TTY terminal session. If you run commands in a non-TTY
agent exec context, use scripted smoke mode instead.

For standard automated verification (tests first, optional smoke):
```bash
scripts/agent_verify.sh
scripts/agent_verify.sh --with-smoke
```

For asset graph and progression consistency checks:
```bash
cargo run -- --validate-assets
scripts/validate_assets.sh
```

**Step 3 — Read only the docs for your current task**  
The handoff block lists which doc files are relevant. Read those and nothing
else. Do not read the full backlog or unrelated gameplay docs.

**Step 4 — Do the work, then write the next handoff**  
When you finish a task *or* must stop mid-task, **update the
`Last Session Handoff` block** in `current-sprint.md` before ending:
- Tick completed checkboxes
- Describe exactly where you stopped if mid-task
- List every file you modified
- Write the next concrete action for the incoming agent

**Step 5 — Move completed tasks**  
When you finish a full task:
1. Prepend a completion record to [tasks/done.md](tasks/done.md)
2. Mark the row ✅ in [tasks/backlog.md](tasks/backlog.md)
3. Pull the next item: find its spec in [tasks/milestones/mXX.md](tasks/milestones/)
4. Copy the spec's "Done When" criteria into the Active Task block of `current-sprint.md`

> **File size discipline**: milestone specs live in  
> `docs/tasks/milestones/mXX.md` — one file per milestone. An agent  
> working on M27 only needs to open `milestones/m27.md`. The backlog  
> is now just an index table (no inline specs after M24).

---

## What Is This Project?

A terminal-based (TUI) open-world Dungeon & Dragons game written in **Rust**
using [Ratatui](https://ratatui.rs/). The game follows the **D&D 5e SRD**
ruleset, features **hand-crafted regions** (the world is split into small
self-contained region files), **branching quests + emergent faction events**
for storytelling, and all three delivery channels: NPC dialog, player journal,
and environmental lore text.

**Tech stack:** Rust 2021 · Ratatui 0.29 · Crossterm · Serde + TOML · Rand

---

## Project Structure (top-level)

```
onebit-dungeons-and-dragons/
├── Cargo.toml
├── assets/          ← all hand-crafted game content (TOML)
├── src/             ← Rust source code
└── docs/            ← you are here
```

For the full source tree, see → [architecture/overview.md](architecture/overview.md)
For doc cross-links and anti-stale update rules, see → [DOCS_MAP.md](DOCS_MAP.md)

---

## Current Status

**→ Check [tasks/current-sprint.md](tasks/current-sprint.md) for what to work on right now.**

Full backlog: [tasks/backlog.md](tasks/backlog.md)  
Completed milestones: [tasks/done.md](tasks/done.md)

---

## Documentation Sync Rule

If you update any gameplay/architecture/content/task doc, review
[DOCS_MAP.md](DOCS_MAP.md) and update linked files in the same change.

---

## System Map — Where to Find Details

| System | Doc |
|---|---|
| App loop / event handling | [architecture/game-loop.md](architecture/game-loop.md) |
| UI / Ratatui screen layout | [architecture/ui-layer.md](architecture/ui-layer.md) |
| TUI visual style (color/icons/animation) | [architecture/tui-visual-system.md](architecture/tui-visual-system.md) |
| Data pipeline (assets → game) | [architecture/data-pipeline.md](architecture/data-pipeline.md) |
| Dice rolling (DiceExpr) | [gameplay/dice.md](gameplay/dice.md) |
| Combat (5e action economy) | [gameplay/combat.md](gameplay/combat.md) |
| Character (stats, class, race) | [gameplay/character.md](gameplay/character.md) |
| World map & region system | [gameplay/world.md](gameplay/world.md) |
| Items & equipment | [gameplay/items.md](gameplay/items.md) |
| Spells & spell slots | [gameplay/spells.md](gameplay/spells.md) |
| NPC AI & monster turns | [gameplay/npc-ai.md](gameplay/npc-ai.md) |
| Story: WorldState & quest machine | [gameplay/story.md](gameplay/story.md) |
| Dialog trees | [gameplay/dialog.md](gameplay/dialog.md) |
| Journal entries | [gameplay/journal.md](gameplay/journal.md) |
| Playable classes reference | [content/classes.md](content/classes.md) |
| Playable races reference | [content/races.md](content/races.md) |
| Monster reference | [content/monsters.md](content/monsters.md) |
| Spells list | [content/spells-list.md](content/spells-list.md) |
| Items list | [content/items-list.md](content/items-list.md) |
| All quests & outcomes | [content/quests.md](content/quests.md) |
| Lore & environmental text | [content/lore.md](content/lore.md) |
| Region index (world map) | [content/regions/index.md](content/regions/index.md) |
| Map & region file format | [content/map-format.md](content/map-format.md) |
| **Logic & Engine Maps** | **[CODE_MAP.md](CODE_MAP.md)**, **[ENGINE_RULES.md](ENGINE_RULES.md)** |
| **Asset Specs** | **[content/SCHEMAS.md](content/SCHEMAS.md)** |
| Testing & Smoke Scripts | [testing/tui-agent-smoke.md](testing/tui-agent-smoke.md), [testing/WRITING_TESTS.md](testing/WRITING_TESTS.md) |

---

## Looking Up Library Docs

When you need to understand a crate's API, use these tools **in this order**:

### 1. `rusty-man` — best for terminal agents (reads local rustdoc JSON)
```bash
cargo install rusty-man            # one-time install
rusty-man ratatui::Terminal        # look up a type
rusty-man ratatui::Frame           # look up a struct
rusty-man crossterm::event::KeyCode # look up an enum variant
```

### 2. Generate local docs
```bash
cargo doc                          # build docs for all deps into target/doc/
```
Then read the generated HTML at `target/doc/<crate>/index.html`, or parse the
JSON at `target/doc/<crate>.json` (add `--output-format json` to `cargo rustdoc`).

### 3. Fetch from docs.rs (requires network)
```bash
# Fetch a specific type page as plain text:
curl -s https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html | \
  sed 's/<[^>]*>//g' | grep -A 20 "impl Terminal"
```
Or use the agent's web fetch ability against `https://docs.rs/<crate>/latest/<crate>/`.

### 4. Source code (always available)
```bash
# Find the definition in the crate source (in .cargo/registry):
find ~/.cargo/registry/src -name "*.rs" -path "*/ratatui*" | \
  xargs grep -l "struct Terminal" | head -3
```

### Key crate docs pages
| Crate | docs.rs URL |
|---|---|
| ratatui | https://docs.rs/ratatui/latest/ratatui/ |
| crossterm | https://docs.rs/crossterm/latest/crossterm/ |
| serde | https://docs.rs/serde/latest/serde/ |
| toml | https://docs.rs/toml/latest/toml/ |
| rand | https://docs.rs/rand/latest/rand/ |
| anyhow | https://docs.rs/anyhow/latest/anyhow/ |

---

## Conventions

- **File Breakdown & Connectivity**: Strive to break logic down into small, highly-focused files. However, a file must never be an "isolated node"—it must have at least one clear inbound reference (e.g., a module declaration or usage) and one clear outbound reference (e.g., an import or logic call) to ensure the code remains a cohesive graph.
- **One concern per module**: `src/ui/` renders only; `src/game/` holds all logic. They never cross-import.
- **Assets are TOML**: All hand-crafted content lives in `assets/`. No content is hardcoded in Rust.
- **Region isolation**: Each world region has its own folder under `assets/regions/<region-slug>/`. An agent working on one region reads only that region's files.
- **WorldState flags**: Story conditions are boolean/integer flags in `WorldState`. No story code mutates character stats directly.
- **ADRs**: Settled architecture decisions are in `docs/decisions/`. Do not re-litigate them.
- **Updating tasks**: When you finish a task, move it from `current-sprint.md` to `done.md` and pull the next item from `backlog.md`.
- **Sample Syncing**: ⚠️ `src/app/samples.rs` contains hardcoded fallback data used by unit tests. If you change a core data structure (e.g. adding a field to `ItemDef`), you MUST update the samples in this file or unit tests will fail or become unreliable.

