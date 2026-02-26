# Regions Index

> One entry per world region. To work on a specific region, find its slug here
> and open **only** `assets/regions/<slug>/`. Do not load other region folders.

---

## Region List

| # | Slug | Name | Status | Type | Weather | Connections |
|---|---|---|---|---|---|---|
| 1 | `valley-of-ash` | Valley of Ash | ✅ complete | volcanic | ash | local rooms |
| 2 | `whispering-woods` | The Whispering Woods | ✅ complete | forest | fog | ← valley-of-ash |
| 3 | `emberpeak-summit` | Emberpeak Summit | ✅ complete | mountain | none | ← valley-of-ash, → ironhold-mines |
| 4 | `ironhold-mines` | The Ironhold Mines | ✅ complete | underground | none | ← emberpeak-summit |
| 5 | `ruined-ironhold-mines` | Ruined Ironhold Mines | ✅ complete | underground | none | ← emberpeak-summit (epic) |
| 6 | `sunken-city` | The Sunken City | ✅ complete | underwater | rain | ← whispering-woods |
| 7 | `underdark-shelf` | The Underdark Shelf | 🔲 planned | underground | none | ← ironhold-mines |
| 8 | `tidewatch-coast` | Tidewatch Coast | ✅ complete | coastal | clear | standalone starter alt |

Status keys: 🔲 planned · 🚧 in progress · ✅ complete

Validation helper:
- `scripts/validate_content.sh` validates authored regions and quests load via the data loader.

Authoring templates:
- `docs/content/regions/templates/region-template.toml`
- `docs/content/regions/templates/room-template.toml`
- `docs/content/regions/templates/npc-template.toml`
- `docs/content/regions/templates/dialog-template.toml`

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
**Key rooms (M21):** Ash Gate · Ember Square · Cinder Ridge · Ash Hollow · Soot Shrine  
**Branching:** Ember Square → Ash Gate (main) or → Cinder Ridge (side branch)  
**Main quest tie-in:** Act 1 starts here (The Bandit King)  
**Region-specific faction:** Ash Bandits (initially hostile)  
**Region type:** volcanic  
**Weather:** ash

## The Whispering Woods (`whispering-woods`)

**Tone:** Dense, ancient forest where trees murmur secrets.  
**Entry point:** Woods Edge from Ash Hollow.  
**Key rooms:** Woods Edge · Sunken Glade · Elder Canopy · Overgrown Path · Cave Entrance  
**Branching:** Woods Edge → Cave Entrance (main) or → Elder Canopy (side)  
**Main quest tie-in:** Act 1 — ancient artifacts  
**Region-specific faction:** Forest Spirits (neutral)  
**Region type:** forest  
**Weather:** fog

## Emberpeak Summit (`emberpeak-summit`)

**Tone:** Cold, volcanic, thin air. Dwarf ruins.  
**Entry point:** South Slope from valley.  
**Key rooms (M21):** Obsidian Bridge · South Slope · Summit Crater · Lava Shelf · Peak Crater  
**Branching:** South Slope → Obsidian Bridge (main) or → Lava Shelf (side branch)  
**Main quest tie-in:** Act 2 — source of the volcanic curse  
**Region-specific faction:** Emberpeak Dwarves (neutral → friendly via quests)  
**Region type:** mountain  
**Weather:** none

## The Ironhold Mines (`ironhold-mines`)

**Tone:** Dark, claustrophobic. Echoing pickaxes, distant growls.  
**Entry point:** Mine Entrance from summit.  
**Key rooms (M21):** Mine Entrance · Deep Shaft · Ore Chamber · Flooded Pit  
**Branching:** Mine Entrance → Deep Shaft (main) or → Ore Chamber (side branch)  
**Main quest tie-in:** Act 2 — bandit king's treasure is hidden here  
**Region-specific faction:** Deep Gnomes (neutral)  
**Region type:** underground  
**Weather:** none

## Ruined Ironhold Mines (`ruined-ironhold-mines`)

**Tone:** Dark, overrun by antagonist forces.  
**Entry point:** Mine Entrance (epic variant).  
**Key rooms:** Mine Entrance · Deep Shaft · Ore Chamber · Flooded Pit  
**Main quest tie-in:** Act 3 — antagonist's lair  
**Region-specific faction:** Enemy forces (hostile)  
**Region type:** underground  
**Weather:** none

## The Sunken City (`sunken-city`)

**Tone:** Drowned metropolis, coral-encrusted spires.  
**Entry point:** Flooded Plaza from Whispering Woods.  
**Key rooms:** Flooded Plaza · Submerged Aqueduct · Drowned Temple  
**Branching:** Flooded Plaza → Drowned Temple (main)  
**Main quest tie-in:** Act 2 — aquatic artifact  
**Region-specific faction:** Aquatic spirits (neutral)  
**Region type:** underwater  
**Weather:** rain

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
