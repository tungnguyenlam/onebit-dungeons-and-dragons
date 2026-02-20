# Regions Index

> One entry per world region. To work on a specific region, find its slug here
> and open **only** `assets/regions/<slug>/`. Do not load other region folders.

---

## Region List

| # | Slug | Name | Status | Connections |
|---|---|---|---|---|
| 1 | `valley-of-ash` | Valley of Ash | ✅ complete | local rooms (`ash_gate` ↔ `ember_square`) |
| 2 | `emberpeak-summit` | Emberpeak Summit | 🔲 planned | ← `valley-of-ash`, → `ironhold-mines` |
| 3 | `ironhold-mines` | The Ironhold Mines | 🔲 planned | ← `emberpeak-summit`, → `underdark-shelf` |
| 4 | `underdark-shelf` | The Underdark Shelf | 🔲 planned | ← `ironhold-mines` |
| 5 | `tidewatch-coast` | Tidewatch Coast | 🔲 planned | standalone starter alt |

Status keys: 🔲 planned · 🚧 in progress · ✅ complete

---

## Per-Region Detail

Each region's **conceptual brief** lives here (one section below).
**Implementation details** (room layouts, NPCs, dialog) live exclusively in:
```
assets/regions/<slug>/
docs/content/regions/<slug>.md   ← author notes, not required for code
```

---

## Valley of Ash (`valley-of-ash`)

**Tone:** Desolate post-war wasteland. Ash drifts constantly.  
**Entry point:** The Ash Gate — a ruined city gate, 1 guard NPC (Kael).  
**Key rooms:** Ash Gate · Ash Plain · Burned Village · Bandit Outpost  
**Main quest tie-in:** Act 1 starts here (The Bandit King)  
**Region-specific faction:** Ash Bandits (initially hostile)

## Emberpeak Summit (`emberpeak-summit`)

**Tone:** Cold, volcanic, thin air. Dwarf ruins.  
**Entry point:** South Slope from valley.  
**Key rooms:** South Slope · Obsidian Bridge · Summit Crater · Dwarven Vault  
**Main quest tie-in:** Act 2 — source of the volcanic curse  
**Region-specific faction:** Emberpeak Dwarves (neutral → friendly via quests)

## The Ironhold Mines (`ironhold-mines`)

**Tone:** Dark, claustrophobic. Echoing pickaxes, distant growls.  
**Entry point:** Mine Entrance from summit.  
**Key rooms:** Mine Entrance · Cart Tunnel · Deep Shaft · The Sump  
**Main quest tie-in:** Act 2 — bandit king's treasure is hidden here  
**Region-specific faction:** Deep Gnomes (neutral)

## The Underdark Shelf (`underdark-shelf`)

**Tone:** Alien. Bioluminescent fungi. Ancient.  
**Entry point:** The Sump from Ironhold.  
**Key rooms:** Fungal Cavern · Drow Outpost · Rift Bridge · The Silence  
**Main quest tie-in:** Act 3 — final confrontation  
**Region-specific faction:** Drow Merchant Coven (hostile unless rep > 5)

## Tidewatch Coast (`tidewatch-coast`)

**Tone:** Maritime, trade town, relatively safe. Good tutorial region.  
**Entry point:** Standalone (alternate starting region).  
**Key rooms:** Dockside · Market Square · The Harbormaster · Sea Cave  
**Main quest tie-in:** Side quest hub  
**Region-specific faction:** Tidewatch Merchants (friendly)
