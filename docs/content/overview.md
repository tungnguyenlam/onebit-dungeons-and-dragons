# Content Overview

> **Index.** Links every content category to its detail doc.
> Authoring a piece of content? Go directly to the relevant detail doc.

---

## What "Content" Means

Content = hand-crafted data files in `assets/`. Rust code does not need to
change when adding new content as long as it conforms to the established TOML
schemas.

---

## Content Index

| Category | Status | Detail doc |
|---|---|---|
| Playable classes | 🔲 planned | [classes.md](classes.md) |
| Playable races | 🔲 planned | [races.md](races.md) |
| Monsters | 🔲 planned | [monsters.md](monsters.md) |
| Spells list | 🔲 planned | [spells-list.md](spells-list.md) |
| Items list | 🔲 planned | [items-list.md](items-list.md) |
| Quests | 🔲 planned | [quests.md](quests.md) |
| Lore & environmental text | 🔲 planned | [lore.md](lore.md) |
| World regions | 🔲 planned | [regions/index.md](regions/index.md) |
| Map & TOML format | ✅ documented | [map-format.md](map-format.md) |

Status keys: 🔲 planned · 🚧 in progress · ✅ complete

---

## Authoring Rules

1. **Schema first**: before adding a new content file type, confirm the serde
   struct exists in `src/data/types.rs`.
2. **Region isolation**: region-specific NPCs and dialog go in
   `assets/regions/<slug>/` — not in the global `assets/` folders.
3. **IDs are kebab-case**: `iron-longsword`, `fireball`, `guard-kael`.
4. **No hardcoded numbers in Rust**: add a new TOML field rather than special-
   casing values in code.

---

## Related Indexes

- Gameplay systems index → [../gameplay/overview.md](../gameplay/overview.md)
- Architecture/data index → [../architecture/overview.md](../architecture/overview.md)
- Active tasks for content milestones → [../tasks/current-sprint.md](../tasks/current-sprint.md)
- Documentation link map → [../DOCS_MAP.md](../DOCS_MAP.md)
