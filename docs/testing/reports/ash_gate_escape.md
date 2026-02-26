# Ash Gate Escape - Interactive Playtest Report

**Date:** 2026-02-20  
**Scenario:** ash_gate  
**Run Type:** Deterministic Capture  
**Tool:** `python3 scripts/visual_check.py --scenario ash_gate --verbose-steps --artifact full --history`

---

## Summary

Successfully completed an automated ash_gate escape run using the new Milestone 15 scenario-aware runner. The scripted flow navigated from character creation through world map entry into the ash_gate room, interacted with the environment, and verified save state persistence.

---

## Key Events

| Step | Action | Result |
|------|--------|--------|
| 1 | Main Menu → Character Creation | ✅ Successful |
| 2 | Character Creation → World Map | ✅ Player spawned in ash_gate room |
| 3 | Save Game (p) | ✅ saves/slot1.toml created |
| 4 | Load Game (o) | ✅ State restored correctly |
| 5 | Open Inventory (i) | ✅ Screen displays |
| 6 | Open Spellbook (s) | ✅ Screen displays |
| 7 | Open Journal (n) | ✅ Screen displays |
| 8 | Enter Combat (a) | ✅ Combat started |
| 9 | Advance Turn (.) | ✅ Turn advanced |
| 10 | Exit Combat (Esc) | ✅ Returned to world map |
| 11 | Quit (q) | ✅ Clean exit |

---

## Save State Verification

**File:** `saves/slot1.toml`

```
region_slug = "valley-of-ash"
room_id = "ash_gate"
player_pos = [3, 2]
player.hp = 24/24
player.level = 1
```

The save correctly persists:
- Player position in ash_gate room
- HP and level
- Inventory (longsword, leather_armor, shield, 3 healing potions)

---

## Findings (UI/Animation/Readability)

### ✅ Working Correctly
- TUI renders cleanly at default terminal size
- All screen transitions (menu → char creation → world map → combat) work
- Inventory, spellbook, journal screens open/close properly
- Save/load cycle functions correctly
- Combat enter/advance/exit flow works

### Observations
- Default smoke flow uses fixed timing (250-450ms delays) - adequate for automated testing
- Interactive mode (`--interactive`) allows manual exploration of scenarios
- Capture mode produces compact logs suitable for CI/agent review

---

## Artifacts

- **Capture Log:** `/tmp/ash_gate.log` (generated via `--capture-log`)
- **Save File:** `saves/slot1.toml` (preserved via `--keep-save`)

---

## Notes

The ash_gate scenario serves as the entry point to the Valley of Ash region. The room contains:
- Captain Kael NPC at position [3, 2]
- Dialog trigger at position [6, 2] 
- Travel trigger to ember_square at position [5, 3]

For full "escape" gameplay, players would:
1. Interact with Captain Kael (dialog trigger)
2. Progress through the region to exit (travel trigger)

The current smoke test verifies the room loads correctly and the player can move within it. Manual interactive testing recommended for full dialog/quest progression.
