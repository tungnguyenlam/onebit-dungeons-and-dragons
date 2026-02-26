# Asset Schemas

Use these examples to author new content. All files are TOML.

## Item (`assets/items/*.toml`)
```toml
id = "bit_shift_rapier"
name = "Bit-Shift Rapier"
item_type = "weapon"
weight = 2.0
value_gp = 25
description = "A shimmering blade that flickers in and out of phase."

[weapon]
damage = "1d8"
damage_type = "piercing"
properties = ["finesse"]

[bonuses]
attack_bonus = 1
```

## Monster (`assets/monsters/*.toml`)
```toml
id = "goblin"
name = "Goblin"
cr = 0.25
size = "small"
monster_type = "humanoid"
alignment = "neutral_evil"
hp = "2d6"
ac = 13
speed = 30
str_score = 8
dex_score = 14
con_score = 10
int_score = 10
wis_score = 8
cha_score = 8
xp = 50

[[actions]]
name = "Scimitar"
description = "Melee Weapon Attack"
attack_bonus = 4
damage = "1d6+2"
damage_type = "slashing"
```

## Quest (`assets/quests/main/*.toml`)
```toml
id = "obsidian_scourge"
name = "The Obsidian Scourge"

[[stages]]
id = "intro"
label = "Investigate the Corruption"
journal_entry = "Speak with Elder Vaelen."
next = [{ condition = "flag:has_eye", stage = "boss_1" }]
```

## Room (`assets/regions/<slug>/rooms/*.toml`)
```toml
id = "ash_gate"
name = "Ash Gate"
description = "A broken stone gate."
landmark = "Ash Gate Landmark"
grid = """
##############
#............#
#..@.........#
##############
"""

[[npcs]]
id = "captain_kael"
position = [3, 2]

[[triggers]]
position = [6, 2]
type = "dialog"
target_id = "captain_kael"
```
