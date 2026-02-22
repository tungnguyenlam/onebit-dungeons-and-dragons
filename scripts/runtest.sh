#!/usr/bin/env bash
# runtest.sh - Step-through testing script for OneBit D&D
#
# Usage:
#   ./runtest.sh              # Start game, freeze at main menu (waits for key)
#   ./runtest.sh n           # Press 'n' (new game), render, exit
#   ./runtest.sh j           # Press 'j' (move down), render, exit
#   ./runtest.sh -h          # Show this help
#
# This script runs the game in --step mode where:
#   1. Game renders one frame
#   2. Waits for a single keypress from stdin
#   3. Processes that key
#   4. Renders the next frame
#   5. Exits
#
# This allows agents to test the game one input at a time, seeing the
# output after each action.
#
# NOTE: This requires a terminal. If running headless, use:
#   echo "n" | ./runtest.sh
#
# Examples:
#   # Start game at main menu
#   ./runtest.sh
#
#   # Create new character (n = new game, then select options)
#   echo -n "n" | ./runtest.sh
#
#   # Move around in game
#   ./runtest.sh j  # move down
#   ./runtest.sh l  # move right
#   ./runtest.sh k  # move up
#   ./runtest.sh h  # move left
#
#   # Open menus
#   ./runtest.sh i  # inventory
#   ./runtest.sh s  # spellbook
#   ./runtest.sh n  # journal
#   ./runtest.sh m  # map
#   ./runtest.sh ?  # help/legend
#
#   # Combat
#   ./runtest.sh a  # attack
#   ./runtest.sh .  # wait

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

show_help() {
    cat << 'EOF'
runtest.sh - Step-through testing script for OneBit D&D

USAGE:
    ./runtest.sh [KEY]

ARGUMENTS:
    KEY     Single character key to press. If not provided, waits for stdin.

EXAMPLES:
    # Start game at main menu (waits for input)
    ./runtest.sh

    # Press 'n' for new game
    echo -n "n" | ./runtest.sh
    ./runtest.sh n

    # Move around (vim-style keys)
    ./runtest.sh j  # move down
    ./runtest.sh k  # move up
    ./runtest.sh h  # move left
    ./runtest.sh l  # move right

    # Actions
    ./runtest.sh a   # attack
    ./runtest.sh .   # wait
    ./runtest.sh i   # inventory
    ./runtest.sh s   # spellbook
    ./runtest.sh n   # journal
    ./runtest.sh m   # map
    ./runtest.sh ?   # help/legend
    ./runtest.sh p   # save game
    ./runtest.sh o   # load game
    ./runtest.sh q   # quit

    # Dialog choices
    ./runtest.sh 1   # choice 1
    ./runtest.sh 2   # choice 2

NOTES:
    - If no KEY provided, reads from stdin (blocks until input)
    - Use 'echo -n' to pass key without newline
    - Game state persists in save.toml

EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    show_help
    exit 0
fi

# Build if needed
if [[ ! -f "target/debug/dnd" ]]; then
    echo "[runtest] Building binary..."
    cargo build --quiet 2>/dev/null || cargo build
fi

KEY="${1:-}"

# Map friendly keywords for AI agents that struggle with terminal escapes
if [[ "$KEY" == "enter" || "$KEY" == "return" ]]; then KEY=$'\r'; fi
if [[ "$KEY" == "esc" || "$KEY" == "escape" ]]; then KEY=$'\x1B'; fi
if [[ "$KEY" == "space" ]]; then KEY=$' '; fi

mkdir -p test_outputs
OUT_FILE="test_outputs/current_screen.txt"

# Disable quick exit to manually capture the engine failure logic
set +e

if [[ "$KEY" == "reset" ]]; then
    rm -f save.toml
    echo "[runtest] Deleted save file. Game reset."
    cargo run --quiet -- --text > "$OUT_FILE"
    CARGO_EXIT=$?
elif [[ -n "$KEY" ]]; then
    # Pass key as argument
    cargo run --quiet -- --text --step -k "$KEY" > "$OUT_FILE"
    CARGO_EXIT=$?
else
    # No key provided - just dump initial state
    cargo run --quiet -- --text > "$OUT_FILE"
    CARGO_EXIT=$?
fi

set -e

if [[ $CARGO_EXIT -ne 0 ]]; then
    echo "❌ ERROR: Game engine crashed or failed to run. See the stack trace above."
    exit 1
fi

echo "✅ Action completed."
echo "🖼️  Full 88x24 TUI screen:"
cat "$OUT_FILE"
