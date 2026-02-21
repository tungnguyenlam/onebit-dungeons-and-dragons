# Step-Through Testing

This document describes how to use the step-through testing mode to test the game without requiring a TTY terminal.

## Overview

The step-through mode allows agents to:
- Run the game in a headless text mode
- See the game state as plain text after each input
- Test gameplay mechanics one keypress at a time
- No terminal/TTY required

## Usage

### Quick Start

```bash
# View initial game state (main menu)
scripts/runtest.sh

# Press a key and see the result
scripts/runtest.sh j        # move down in menu
scripts/runtest.sh $'\r'    # press Enter
scripts/runtest.sh a        # attack
```

### Command Line Options

```bash
# Start game, dump state as text
cargo run -- --text

# Press a key, dump state
cargo run -- --text --step -k j

# Interactive TUI (requires terminal)
cargo run -- --mode tui

# Step mode with TUI (requires terminal)
cargo run -- --step
```

### Key Mappings

| Key | Action |
|-----|--------|
| `j` | Move down / vim-style down |
| `k` | Move up / vim-style up |
| `h` | Move left / vim-style left |
| `l` | Move right / vim-style right |
| `Enter` | Confirm / select |
| `Space` | Confirm / select |
| `Esc` | Cancel / back |
| `i` | Open inventory |
| `s` | Open spellbook |
| `n` | Open journal |
| `m` | Open world map |
| `?` | Toggle help/legend |
| `p` | Save game |
| `o` | Load game |
| `a` | Attack |
| `.` | Wait |
| `b` | Toggle sound |
| `q` | Quit |
| `1-9` | Dialog choices |

## Example Workflow

### Test Character Creation Flow

```bash
# 1. Start at main menu
scripts/runtest.sh

# 2. Press Enter to select "New Game"
scripts/runtest.sh $'\r'

# 3. See character creation screen
# (then use j/k to navigate, Enter to select class)
```

### Test Gameplay

```bash
# Start fresh
scripts/runtest.sh

# Create new character
scripts/runtest.sh $'\r'       # Enter: New Game

# Select class (navigate with j/k, select with Enter)
scripts/runtest.sh j            # Move to fighter
scripts/runtest.sh $'\r'       # Select fighter

# Select race
scripts/runtest.sh j            # Move to human
scripts/runtest.sh $'\r'       # Select human

# Now in game world - move around
scripts/runtest.sh j            # Move down
scripts/runtest.sh l            # Move right
scripts/runtest.sh k            # Move up

# Open menus
scripts/runtest.sh i            # Inventory
scripts/runtest.sh n            # Journal
scripts/runtest.sh m            # Map

# Combat (if encounter starts)
scripts/runtest.sh a            # Attack
scripts/runtest.sh .            # Wait
```

## Output Format

The text dump now outputs the **exact visual TUI layout** using Ratatui's `TestBackend`. This provides a character-for-character replication of what the human player sees, including all boxes, text, colors (stripped to text), and UI elements for the 88x24 terminal grid.

```text
┌World─────────────────────────────────────────────────────────────────────────────────┐
│ Region: Valley of Ash (valley-of-ash)            Day: 1                              │
│ Room: ash_gate                                   Weather: Ash                        │
│ Player: Theron at (3, 2)                         Threat: None                        │
├Map───────────────────────────────────────────────────────────────────────────────────┤
...
```

## Integration with Agents

This mode is designed for agents to:
1. Verify gameplay mechanics work correctly
2. Test bug fixes
3. Check state changes after actions
4. Validate UI updates

Example agent workflow:
```bash
# Test that pressing 'a' in combat attacks
scripts/runtest.sh           # Start game
scripts/runtest.sh $'\r'    # New game
# ... navigate to combat ...
scripts/runtest.sh a         # Attack
# Check output shows damage dealt
```

## Script: runtest.sh

Location: `scripts/runtest.sh`

Usage:
```bash
./runtest.sh [KEY]

# Examples
./runtest.sh                 # Dump initial state
./runtest.sh j               # Press 'j', dump state
./runtest.sh $'\r'           # Press Enter, dump state
./runtest.sh -h              # Show help
```

## Notes

- The game state persists in `save.toml` between runs
- Use `scripts/runtest.sh -h` for full help
- This mode does not require any terminal/TTY
- Output is plain text directly mirroring the TUI grid, naturally parsed line-by-line.
