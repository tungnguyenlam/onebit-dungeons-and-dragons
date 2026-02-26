# Documentation Link Map

> Purpose: keep docs synchronized by making cross-file dependencies explicit.
> Before merging doc or feature changes, use this file as the review checklist.

---

## Primary Entry Points

- Agent workflow: [AGENT.md](../AGENT.md)
- Architecture index: [architecture/overview.md](architecture/overview.md)
- Gameplay index: [gameplay/overview.md](gameplay/overview.md)
- Content index: [content/overview.md](content/overview.md)
- Testing docs: [testing/tui-agent-smoke.md](testing/tui-agent-smoke.md), [testing/WRITING_TESTS.md](testing/WRITING_TESTS.md)
- Knowledge Maps: [CODE_MAP.md](CODE_MAP.md), [ENGINE_RULES.md](ENGINE_RULES.md), [content/SCHEMAS.md](content/SCHEMAS.md)
- Task tracking: [tasks/current-sprint.md](tasks/current-sprint.md), [tasks/backlog.md](tasks/backlog.md), [tasks/done.md](tasks/done.md)

---

## Dependency Map

### Architecture docs

- `architecture/renderer.md`
  - Related: [architecture/game-loop.md](architecture/game-loop.md), [architecture/ui-layer.md](architecture/ui-layer.md), [decisions/adr-001-ratatui-state.md](decisions/adr-001-ratatui-state.md)
- `architecture/game-loop.md`
  - Related: [architecture/renderer.md](architecture/renderer.md), [architecture/ui-layer.md](architecture/ui-layer.md), [tasks/current-sprint.md](tasks/current-sprint.md)
- `architecture/ui-layer.md`
  - Related: [architecture/renderer.md](architecture/renderer.md), [gameplay/combat.md](gameplay/combat.md), [gameplay/dialog.md](gameplay/dialog.md), [gameplay/world.md](gameplay/world.md)
- `architecture/tui-visual-system.md`
  - Related: [architecture/ui-layer.md](architecture/ui-layer.md), [architecture/game-loop.md](architecture/game-loop.md), [testing/tui-agent-smoke.md](testing/tui-agent-smoke.md), [tasks/backlog.md](tasks/backlog.md)
- `architecture/data-pipeline.md`
  - Related: [content/map-format.md](content/map-format.md), [content/overview.md](content/overview.md), [tasks/backlog.md](tasks/backlog.md)

### Gameplay docs

- `gameplay/combat.md`
  - Related: [gameplay/dice.md](gameplay/dice.md), [gameplay/items.md](gameplay/items.md), [gameplay/spells.md](gameplay/spells.md), [architecture/ui-layer.md](architecture/ui-layer.md)
- `gameplay/items.md`
  - Related: [gameplay/spells.md](gameplay/spells.md), [content/items-list.md](content/items-list.md), [tasks/backlog.md](tasks/backlog.md)
- `gameplay/spells.md`
  - Related: [gameplay/character.md](gameplay/character.md), [content/spells-list.md](content/spells-list.md), [tasks/backlog.md](tasks/backlog.md)
- `gameplay/story.md`
  - Related: [gameplay/dialog.md](gameplay/dialog.md), [gameplay/journal.md](gameplay/journal.md), [content/quests.md](content/quests.md), [content/lore.md](content/lore.md)
- `gameplay/world.md`
  - Related: [content/regions/index.md](content/regions/index.md), [content/map-format.md](content/map-format.md), [architecture/ui-layer.md](architecture/ui-layer.md)
- `gameplay/npc-ai.md`
  - Related: [content/monsters.md](content/monsters.md), [gameplay/story.md](gameplay/story.md), [tasks/backlog.md](tasks/backlog.md)

### Content docs

- `content/map-format.md`
  - Related: [content/regions/index.md](content/regions/index.md), [architecture/data-pipeline.md](architecture/data-pipeline.md), [gameplay/world.md](gameplay/world.md)
- `content/monsters.md`
  - Related: [gameplay/npc-ai.md](gameplay/npc-ai.md), [tasks/backlog.md](tasks/backlog.md)
- `content/items-list.md`
  - Related: [gameplay/items.md](gameplay/items.md), [tasks/backlog.md](tasks/backlog.md)
- `content/spells-list.md`
  - Related: [gameplay/spells.md](gameplay/spells.md), [tasks/backlog.md](tasks/backlog.md)
- `content/quests.md`
  - Related: [gameplay/story.md](gameplay/story.md), [tasks/backlog.md](tasks/backlog.md)
- `content/lore.md`
  - Related: [gameplay/story.md](gameplay/story.md), [content/map-format.md](content/map-format.md)

### Task docs

- `tasks/current-sprint.md`
  - Related: [tasks/backlog.md](tasks/backlog.md), [tasks/done.md](tasks/done.md), [AGENT.md](../AGENT.md)
- `tasks/backlog.md`
  - Related: [tasks/current-sprint.md](tasks/current-sprint.md), [tasks/done.md](tasks/done.md), feature docs in `gameplay/` and `content/`
- `tasks/done.md`
  - Related: [tasks/backlog.md](tasks/backlog.md), [tasks/current-sprint.md](tasks/current-sprint.md), [README.md](../README.md)

### Testing docs

- `testing/tui-agent-smoke.md`
  - Related: [AGENT.md](../AGENT.md), [README.md](../README.md), `scripts/agent_tui_smoke.sh`, `scripts/agent_verify.sh`
- `testing/WRITING_TESTS.md`
  - Related: `src/app/tests.rs`, `src/game/combat/attack.rs`

---

## Update Checklist (Anti-Stale)

When editing these files, also review:

- Runtime/UI behavior change:
  - [README.md](../README.md)
  - [architecture/overview.md](architecture/overview.md)
  - [architecture/ui-layer.md](architecture/ui-layer.md)
  - [architecture/tui-visual-system.md](architecture/tui-visual-system.md)
  - relevant docs in [gameplay/overview.md](gameplay/overview.md)
- Data model / TOML schema change:
  - [architecture/data-pipeline.md](architecture/data-pipeline.md)
  - relevant `docs/content/*.md`
  - task docs in `docs/tasks/`
- Milestone/task status change:
  - [tasks/current-sprint.md](tasks/current-sprint.md)
  - [tasks/backlog.md](tasks/backlog.md)
  - [tasks/done.md](tasks/done.md)
  - status table in [README.md](../README.md)
