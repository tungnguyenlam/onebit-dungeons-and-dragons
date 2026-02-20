# Monsters

> Add each monster as `assets/monsters/<slug>.toml`.
> Full stat block schema in [gameplay/npc-ai.md](../gameplay/npc-ai.md).

---

## Planned Monsters (priority order)

| Slug | Name | CR | Type | AI Behaviour | Status |
|---|---|---|---|---|---|
| `goblin` | Goblin | 1/4 | Humanoid | skirmisher | 🔲 |
| `skeleton` | Skeleton | 1/4 | Undead | brute | 🔲 |
| `zombie` | Zombie | 1/4 | Undead | brute | 🔲 |
| `city-guard` | City Guard | 1/8 | Humanoid | brute | 🔲 |
| `bandit` | Bandit | 1/8 | Humanoid | skirmisher | 🔲 |
| `wolf` | Wolf | 1/4 | Beast | brute | 🔲 |
| `giant-spider` | Giant Spider | 1 | Beast | skirmisher | 🔲 |
| `orc` | Orc | 1/2 | Humanoid | brute | 🔲 |
| `hobgoblin` | Hobgoblin | 1/2 | Humanoid | brute | 🔲 |
| `gnoll` | Gnoll | 1/2 | Humanoid | brute | 🔲 |
| `dire-wolf` | Dire Wolf | 1 | Beast | brute | 🔲 |
| `owlbear` | Owlbear | 3 | Monstrosity | brute | 🔲 |
| `troll` | Troll | 5 | Giant | brute | 🔲 |
| `bandit-king` | Bandit King | 4 | Humanoid | skirmisher | 🔲 |
| `drow-warrior` | Drow Warrior | 2 | Humanoid | ranged | 🔲 |
| `fire-elemental` | Fire Elemental | 5 | Elemental | brute | 🔲 |

---

## Notes

- `city-guard` is the base for region NPCs like `guard-kael` (overridden via
  `[overrides]` in the region's `npcs/<id>.toml`).
- Named boss monsters (e.g. `bandit-king`) get unique dialog trees and may
  have multiple combat phases (implement as separate action lists with HP
  threshold triggers).
