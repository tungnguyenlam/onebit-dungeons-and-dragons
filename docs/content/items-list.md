# Items List

> Item TOML files live in `assets/items/<id>.toml`.
> Full schema in [gameplay/items.md](../gameplay/items.md).

---

## Weapons

| ID | Name | Damage | Type | Properties | Status |
|---|---|---|---|---|---|
| `dagger` | Dagger | 1d4 | piercing | finesse, thrown (20/60) | 🔲 |
| `handaxe` | Handaxe | 1d6 | slashing | light, thrown (20/60) | 🔲 |
| `shortsword` | Shortsword | 1d6 | piercing | finesse, light | 🔲 |
| `longsword` | Longsword | 1d8 / 1d10 | slashing | versatile | 🔲 |
| `greataxe` | Greataxe | 1d12 | slashing | heavy, two-handed | 🔲 |
| `greatsword` | Greatsword | 2d6 | slashing | heavy, two-handed | 🔲 |
| `shortbow` | Shortbow | 1d6 | piercing | ranged (80/320) | 🔲 |
| `longbow` | Longbow | 1d8 | piercing | heavy, ranged (150/600) | 🔲 |
| `crossbow-light` | Light Crossbow | 1d8 | piercing | ranged (80/320), loading | 🔲 |
| `quarterstaff` | Quarterstaff | 1d6 / 1d8 | bludgeoning | versatile | 🔲 |
| `rapier` | Rapier | 1d8 | piercing | finesse | 🔲 |

---

## Armor

| ID | Name | AC | Type | Stealth | Status |
|---|---|---|---|---|---|
| `leather-armor` | Leather Armor | 11 + DEX | light | normal | 🔲 |
| `studded-leather` | Studded Leather | 12 + DEX | light | normal | 🔲 |
| `chain-shirt` | Chain Shirt | 13 + DEX (max 2) | medium | normal | 🔲 |
| `breastplate` | Breastplate | 14 + DEX (max 2) | medium | normal | 🔲 |
| `half-plate` | Half Plate | 15 + DEX (max 2) | medium | disadvantage | 🔲 |
| `chain-mail` | Chain Mail | 16 | heavy | disadvantage | 🔲 |
| `plate` | Plate | 18 | heavy | disadvantage | 🔲 |
| `shield` | Shield | +2 | shield | normal | 🔲 |

---

## Consumables

| ID | Name | Effect | Status |
|---|---|---|---|
| `potion-healing` | Potion of Healing | Heal 2d4+2 HP | 🔲 |
| `potion-greater-healing` | Greater Healing Potion | Heal 4d4+4 HP | 🔲 |
| `scroll-fireball` | Scroll of Fireball | Cast Fireball once (DC 13) | 🔲 |
| `antitoxin` | Antitoxin | Advantage on CON saves vs poison for 1 hr | 🔲 |
| `arrows-20` | Arrows (20) | Ammunition for bows | 🔲 |
| `bolts-20` | Bolts (20) | Ammunition for crossbows | 🔲 |

---

## Quest Items

Quest items have `type = "quest"` and cannot be dropped or sold.

| ID | Name | Used in quest | Status |
|---|---|---|---|
| `captains-letter` | Captain's Letter | bandit-king (Act 1) | 🔲 |
| `ember-artefact` | The Ember Artefact | volcanic-curse (Act 2) | 🔲 |
| `drow-contract` | Drow Silk Contract | tidewatch-smugglers | 🔲 |
