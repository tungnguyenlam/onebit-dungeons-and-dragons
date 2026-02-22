# Step-Through Testing

This document describes how to use the step-through testing mode to test the game without requiring a TTY terminal.

## Overview

The step-through mode allows agents to:
- Run the game in a headless text mode
- See the game state as plain text after each input
- Test gameplay mechanics one keypress at a time
- No terminal/TTY required
- **Friendly Keywords**: Supports `enter`, `esc`, `space`, and `reset` for easier automation.
- **Persistence**: Automatically saves the full TUI screen to `test_outputs/current_screen.txt`.

## Usage

### Quick Start

The `scripts/runtest.sh` script is the primary tool for headless testing.

```bash
# View initial game state (main menu)
scripts/runtest.sh

# Press a key and see the result
scripts/runtest.sh j        # move down in menu
scripts/runtest.sh enter    # press Enter (uses friendly keyword)
scripts/runtest.sh a        # attack
scripts/runtest.sh reset    # Reset game state (deletes save.toml and restarts)
```

### Friendly Keywords for Agents

To simplify automation, `runtest.sh` maps the following keywords to their terminal escape sequences:

| Keyword | Mapping |
|---------|---------|
| `enter` | `\r` (Return) |
| `return`| `\r` (Return) |
| `esc`   | `\x1B` (Escape) |
| `escape`| `\x1B` (Escape) |
| `space` | ` ` (Spacebar) |
| `reset` | Deletes `save.toml` and runs the engine fresh |

### Key Mappings (Standard)

| Key | Action |
|-----|--------|
| `j` | Move down / vim-style down |
| `k` | Move up / vim-style up |
| `h` | Move left / vim-style left |
| `l` | Move right / vim-style right |
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

## Output Format

The tool provides two ways to view the state:
1. **Direct Stdout**: The full 88x24 TUI grid is printed directly to your terminal/context.
2. **File Backup**: Every run saves the output to `test_outputs/current_screen.txt`.

```text
✅ Action completed.
🖼️  Full 88x24 TUI screen:
┌World─────────────────────────────────────────────────────────────────────────────────┐
│ Region: Valley of Ash (valley-of-ash)            Turn: 5                             │
...
```

## Integration with Agents

### Error Handling
The `runtest.sh` script includes robust error detection. If the game engine crashes (e.g., panics or fails to compile), the script will:
1. Detect the non-zero exit code.
2. Report `❌ ERROR: Game engine crashed or failed to run.`
3. Exit with status 1.

This prevents agents from hallucinating a successful action when the engine has actually failed.

### Workflow Example
```bash
# 1. Start fresh
scripts/runtest.sh reset

# 2. Enter New Game menu
scripts/runtest.sh enter

# 3. Choose starting options
scripts/runtest.sh j
scripts/runtest.sh enter
```

## Notes

- The game state persists in `save.toml` between runs (unless `reset` is used).
- This mode uses Ratatui's `TestBackend` to provide a character-for-character replication of the TUI.
- All compiler warnings are suppressed in `runtest.sh` output to keep agent context clean.
