#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TIMEOUT_SECONDS="${TUI_TIMEOUT:-120}"
WITH_TESTS=0
KEEP_SAVE=0
NO_BUILD=0
INTERACTIVE=0
CAPTURE_LOG=""
BUILD_RUSTFLAGS="${TUI_RUSTFLAGS:--Awarnings}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-tests)
      WITH_TESTS=1
      shift
      ;;
    --keep-save)
      KEEP_SAVE=1
      shift
      ;;
    --no-build)
      NO_BUILD=1
      shift
      ;;
    --interactive)
      INTERACTIVE=1
      shift
      ;;
    --capture-log)
      CAPTURE_LOG="$2"
      shift 2
      ;;
    --timeout)
      TIMEOUT_SECONDS="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: scripts/agent_tui_smoke.sh [--with-tests] [--keep-save] [--no-build] [--interactive] [--capture-log <file>] [--timeout <seconds>]"
      exit 2
      ;;
  esac
done

if ! command -v expect >/dev/null 2>&1; then
  echo "Error: 'expect' is required for automated TUI keyboard driving."
  echo "Install with: brew install expect"
  exit 1
fi

if [[ "$NO_BUILD" -eq 0 ]]; then
  echo "[agent-tui] Building binary..."
  RUSTFLAGS="$BUILD_RUSTFLAGS" cargo build --quiet --bin dnd
fi

if [[ "$INTERACTIVE" -eq 1 ]]; then
  echo "[agent-tui] Interactive mode: launching ./target/debug/dnd --mode tui"
  exec ./target/debug/dnd --mode tui
fi

rm -f saves/slot1.toml

echo "[agent-tui] Running scripted TUI smoke flow..."
export TUI_CAPTURE_LOG="$CAPTURE_LOG"
TUI_TIMEOUT="$TIMEOUT_SECONDS" expect <<'EXPECT_EOF'
set timeout $env(TUI_TIMEOUT)
log_user 0

if {[info exists env(TUI_CAPTURE_LOG)] && $env(TUI_CAPTURE_LOG) ne ""} {
  # `script` captures full PTY output, which is better for later UI inspection.
  spawn script -q $env(TUI_CAPTURE_LOG) ./target/debug/dnd --mode tui
} else {
  spawn ./target/debug/dnd --mode tui
}

# Main Menu -> Character Creation
after 1100
send "\r"

# Character Creation -> Start Adventure
after 350
send "jjj\r"

# World map: save/load + open/close common screens
after 450
send "p"
after 250
send "o"
after 250
send "i"
after 250
send "\033"
after 250
send "s"
after 250
send "\033"
after 250
send "n"
after 250
send "\033"

# Enter/exit combat quickly
after 250
send "a"
after 350
send "."
after 250
send "\033"

# Quit
after 250
send "q"

expect eof
EXPECT_EOF

if [[ ! -s saves/slot1.toml ]]; then
  echo "[agent-tui] FAILED: save file was not created (expected saves/slot1.toml)."
  exit 1
fi

echo "[agent-tui] Smoke flow passed."
echo "[agent-tui] Save file created at saves/slot1.toml"

if [[ "$WITH_TESTS" -eq 1 ]]; then
  echo "[agent-tui] Running cargo test..."
  cargo test
fi

if [[ "$KEEP_SAVE" -eq 0 ]]; then
  rm -f saves/slot1.toml
  rmdir saves 2>/dev/null || true
fi

echo "[agent-tui] Done."
