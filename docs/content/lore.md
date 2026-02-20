# Lore & Environmental Text

> Lore entries are defined in `assets/lore/<id>.toml`.
> They are triggered by the `lore` trigger type in room files (see [content/map-format.md](map-format.md)).
> Reading a lore item sets `flag:read_<lore-id>` in WorldState and may add a
> journal entry.

---

## Lore TOML Schema

```toml
id            = "ash-wars-pamphlet"
name          = "A Tattered Pamphlet"
category      = "history"      # history | personal | arcane | mundane
journal_entry = true           # adds to journal under Lore category when read
body = """
'THE ASH WARS — A HISTORY'
Thirty years past, the armies of the Ember King marched south through
this valley, leaving nothing but soot and silence. The survivors fled
to Tidewatch. None dared return — until now.
"""
```

---

## Planned Lore Entries

| ID | Name | Region | Category | Status |
|---|---|---|---|---|
| `ash-wars-pamphlet` | A Tattered Pamphlet | Valley of Ash | history | 🔲 |
| `kael-personal-letter` | Kael's Unsent Letter | Valley of Ash | personal | 🔲 |
| `emberpeaks-dwarven-log` | Engineer's Log | Emberpeak Summit | history | 🔲 |
| `ironhold-mining-record` | Mining Record #7 | Ironhold Mines | mundane | 🔲 |
| `bandit-king-manifesto` | The King's Manifesto | Ironhold Mines | personal | 🔲 |
| `drow-contract` | Silk-Script Contract | Underdark Shelf | arcane | 🔲 |
| `silence-prophecy` | The Prophecy of Silence | Underdark Shelf | arcane | 🔲 |
| `tidewatch-ledger` | Smuggler's Ledger Page | Tidewatch Coast | mundane | 🔲 |

---

## Environmental Text (Signs, Engravings)

Short one-line texts on signs, door engravings, or grave markers do **not**
require a TOML entry — they can be inline in the room's `description` field or
as a `lore` trigger with an inline body (no `assets/lore/` file needed for
text under 3 lines).

```toml
# In room.toml triggers section:
[[triggers]]
position  = [5, 3]
type      = "lore"
inline    = true
name      = "A carved inscription"
body      = "HERE LIES THE PRIDE OF THE VALLEY. DO NOT DISTURB."
```
