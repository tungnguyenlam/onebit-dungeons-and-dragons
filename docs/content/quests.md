# Quests

> Quest TOML files live in `assets/quests/main/` and `assets/quests/side/`.
> Schema and stage machine details: [gameplay/story.md](../gameplay/story.md).

---

## Main Story Arc

### Act 1 — The Burning Road

**Start region:** Valley of Ash  
**Quest file:** `assets/quests/main/bandit-king.toml`

| Stage | Trigger condition | Summary |
|---|---|---|
| `start` | Quest accepted from Captain | Investigate burned villages |
| `track` | `flag:found_bandits` | Track bandits to Ironhold Mines |
| `confront` | `flag:entered_bandit_lair` | Defeat or persuade the Bandit King |
| `done_kill` | `flag:killed_bandit_lord` | Justice served by sword |
| `done_persuade` | `flag:persuaded_bandit_lord` | Uneasy alliance formed |

**Key WorldState flags set by this quest:**
- `flag:act1_complete` (unlocks travel to Emberpeak)
- `flag:bandit_king_alive` or `flag:bandit_king_dead`

---

### Act 2 — The Volcanic Curse *(planned)*

**Start region:** Emberpeak Summit  
**Quest file:** `assets/quests/main/volcanic-curse.toml`

| Stage | Summary |
|---|---|
| `start` | Discover source of the eruptions |
| `mines` | Find the cursed artefact in Ironhold |
| `ritual` | Perform or destroy the ritual |
| `done` | Curse lifted or harnessed |

---

### Act 3 — The Silence Below *(planned)*

**Start region:** Underdark Shelf  
**Quest file:** `assets/quests/main/silence-below.toml`

Details TBD pending Act 2 outcomes.

---

## Side Quests

| ID | Name | Region | Giver | Status |
|---|---|---|---|---|
| `missing-merchant` | The Missing Merchant | Valley of Ash | Innkeeper Mara | 🔲 |
| `dwarven_relic` | A Dwarven Relic | Emberpeak Summit | Archivist Nyra | ✅ |
| `gnome_debt` | The Gnome's Debt | Ironhold Mines | Foreman Tarik | ✅ |
| `spider-silk` | Spider Silk for Sable | Ironhold Mines | Merchant Sable | 🔲 |
| `tidewatch-smugglers` | Smuggler's Ledger | Tidewatch Coast | Harbormaster | 🔲 |

---

## WorldState Flag Naming Convention

```
flag:quest_<quest-id>_active      # quest is accepted and in progress
flag:quest_<quest-id>_complete    # quest resolved
flag:<specific_event>             # one-off narrative flags (free-form)
counter:faction_<faction-id>_rep  # signed integer reputation
```
