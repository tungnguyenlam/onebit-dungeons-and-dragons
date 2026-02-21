# Engine Rules & Formulae

One-Bit D&D follows a simplified D&D 5e SRD ruleset.

## Core Formulae

### Combat
- **Attack Roll**: `d20 + StatMod + ProficiencyBonus (if applicable) + ItemBonus`
    - Natural 1: Always Misses.
    - Natural 20: Always Critical (double damage dice, keep modifier).
- **Armor Class (AC)**: `Base (from Armor) + DexMod (capped by armor type) + ShieldBonus + ItemBonus`
    - *Light Armor*: No Dex cap.
    - *Medium Armor*: Max Dex +2.
    - *Heavy Armor*: Max Dex 0.
- **Damage**: `DiceExpr.roll() + StatMod (usually Strength for melee, Dex for ranged)`.
    - Damage is **never** less than 0.
    - **Resistance**: Halves damage (round down).
    - **Vulnerability**: Doubles damage.

### Saving Throws
- **DC (Difficulty Class)**: Usually `8 + CastingStatMod + ProficiencyBonus`.
- **Save Roll**: `d20 + AbilityMod + ProficiencyBonus (if proficient)`.
    - If `Roll >= DC`, Success.

## Progression
- **Proficiency Bonus**:
    - Level 1-4: +2
    - Level 5-8: +3
    - Level 9-12: +4
- **Ability Modifiers**: `floor((score - 10) / 2)`

## Data Conventions
- **DiceExpr**: Represented as strings in assets (`1d8+2`, `2d6`).
- **Rounding**: Unless specified otherwise, always round **down** toward zero.
- **Turn Order**: Determined by Initiative roll (`d20 + DexMod`) at the start of combat.

## Limitations
- **WorldState**: Boolean flags and integer counters only. No string variables.
- **Inventory**: No grid-based inventory. It is a simple weight-limited list.
