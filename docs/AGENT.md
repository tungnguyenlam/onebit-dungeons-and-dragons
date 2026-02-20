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
If you finished a full task: move it to [tasks/done.md](tasks/done.md)
and pull the next item from [tasks/backlog.md](tasks/backlog.md).

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

---

## Current Status

**→ Check [tasks/current-sprint.md](tasks/current-sprint.md) for what to work on right now.**

Full backlog: [tasks/backlog.md](tasks/backlog.md)  
Completed milestones: [tasks/done.md](tasks/done.md)

---

## System Map — Where to Find Details

| System | Doc |
|---|---|
| App loop / event handling | [architecture/game-loop.md](architecture/game-loop.md) |
| UI / Ratatui screen layout | [architecture/ui-layer.md](architecture/ui-layer.md) |
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

- **One concern per module**: `src/ui/` renders only; `src/game/` holds all logic. They never cross-import.
- **Assets are TOML**: All hand-crafted content lives in `assets/`. No content is hardcoded in Rust.
- **Region isolation**: Each world region has its own folder under `assets/regions/<region-slug>/`. An agent working on one region reads only that region's files.
- **WorldState flags**: Story conditions are boolean/integer flags in `WorldState`. No story code mutates character stats directly.
- **ADRs**: Settled architecture decisions are in `docs/decisions/`. Do not re-litigate them.
- **Updating tasks**: When you finish a task, move it from `current-sprint.md` to `done.md` and pull the next item from `backlog.md`.
