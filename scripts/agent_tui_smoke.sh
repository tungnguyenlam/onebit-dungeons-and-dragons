#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"


# Scenario presets
SCENARIO=""
MAX_FRAMES=""
TOKEN_EFFICIENT=0
TIMEOUT_SECONDS="${TUI_TIMEOUT:-120}"
SOAK=0
SOAK_PROFILE="standard"
SOAK_MINUTES=0
WITH_TESTS=0
KEEP_SAVE=0
NO_BUILD=0
INTERACTIVE=0
CAPTURE_LOG=""
BUILD_RUSTFLAGS="${TUI_RUSTFLAGS:--Awarnings}"


usage() {
  echo "Usage: scripts/agent_tui_smoke.sh [--with-tests] [--keep-save] [--no-build] [--interactive] [--capture-log <file>] [--timeout <seconds>] [--scenario <name>] [--max-frames <n>] [--token-efficient] [--soak] [--profile <name>] [--minutes <n>] [--list-scenarios]"
  exit 2
}

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
    --scenario)
      SCENARIO="$2"
      shift 2
      ;;
    --max-frames)
      MAX_FRAMES="$2"
      shift 2
      ;;
    --token-efficient)
      TOKEN_EFFICIENT=1
      shift
      ;;
    --soak)
      SOAK=1
      shift
      ;;
    --profile)
      SOAK_PROFILE="$2"
      shift 2
      ;;
    --minutes)
      SOAK_MINUTES="$2"
      shift 2
      ;;
    --list-scenarios)
      echo "ash_gate"
      echo "ember_square"
      echo "river_watch"
      exit 0
      ;;
    *)
      usage
      ;;
  esac
done

if [[ "$NO_BUILD" -eq 0 ]]; then
  echo "[agent-tui] Building binary..."
  RUSTFLAGS="$BUILD_RUSTFLAGS" cargo build --quiet --bin dnd
fi


# Scenario preset logic
if [[ -n "$SCENARIO" ]]; then
  case "$SCENARIO" in
    ash_gate)
      # Preset for ash_gate scenario
      ;;
    ember_square)
      ;;
    river_watch)
      ;;
    *)
      echo "Unknown scenario: $SCENARIO"
      exit 2
      ;;
  esac
fi

if [[ "$INTERACTIVE" -eq 1 ]]; then
  if [[ ! -t 0 || ! -t 1 ]]; then
    echo "Error: --interactive requires a TTY (stdin/stdout must be terminals)."
    echo "Run this directly in a terminal, or omit --interactive for scripted smoke mode."
    exit 2
  fi
  if [[ -n "$SCENARIO" ]]; then
    echo "[agent-tui] Interactive mode with scenario preset: $SCENARIO"
  fi
  echo "[agent-tui] Interactive mode: launching ./target/debug/dnd --mode tui"
  exec ./target/debug/dnd --mode tui
fi

if ! command -v expect >/dev/null 2>&1; then
  echo "Error: 'expect' is required for automated TUI keyboard driving."
  echo "Install with: brew install expect"
  exit 1
fi

rm -f saves/slot1.toml

run_scripted_flow() {
  local scenario="$1"
  local capture_log="$2"

  export TUI_TOKEN_EFFICIENT="$TOKEN_EFFICIENT"
  export TUI_MAX_FRAMES="$MAX_FRAMES"
  export TUI_SCENARIO="$scenario"
  export TUI_CAPTURE_LOG="$capture_log"
  TUI_TIMEOUT="$TIMEOUT_SECONDS" expect <<'EXPECT_EOF'
set timeout $env(TUI_TIMEOUT)
log_user 0

if {[info exists env(TUI_CAPTURE_LOG)] && $env(TUI_CAPTURE_LOG) ne ""} {
  set cmd [list ./target/debug/dnd --mode tui]
  spawn script -q $env(TUI_CAPTURE_LOG) {*}$cmd
} else {
  set cmd [list ./target/debug/dnd --mode tui]
  spawn {*}$cmd
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
}

if [[ "$SOAK" -eq 1 ]]; then
  case "$SOAK_PROFILE" in
    standard) ;;
    *)
      echo "Unknown soak profile: $SOAK_PROFILE"
      exit 2
      ;;
  esac
  if [[ "$SOAK_MINUTES" -le 0 ]]; then
    echo "Error: --soak requires --minutes <n> with n > 0"
    exit 2
  fi

  echo "[agent-tui] Running soak profile '$SOAK_PROFILE' for ${SOAK_MINUTES} minute(s)..."
  start_epoch="$(date +%s)"
  end_epoch="$((start_epoch + SOAK_MINUTES * 60))"
  iteration=0
  scenarios=(ash_gate ember_square river_watch)
  while [[ "$(date +%s)" -lt "$end_epoch" ]]; do
    scenario="${scenarios[$((iteration % ${#scenarios[@]}))]}"
    echo "[agent-tui] Soak iteration $((iteration + 1)) scenario=$scenario"
    run_scripted_flow "$scenario" ""
    iteration=$((iteration + 1))
  done
  echo "[agent-tui] Soak complete: iterations=$iteration profile=$SOAK_PROFILE"
else
  echo "[agent-tui] Running scripted TUI smoke flow..."
  run_scripted_flow "$SCENARIO" "$CAPTURE_LOG"
fi

if [[ "$KEEP_SAVE" -eq 0 ]]; then
  rm -f saves/slot1.toml
  rmdir saves 2>/dev/null || true
fi

echo "[agent-tui] Done."
