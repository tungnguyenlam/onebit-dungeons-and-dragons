#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

WITH_SMOKE=0
KEEP_SAVE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-smoke)
      WITH_SMOKE=1
      shift
      ;;
    --keep-save)
      KEEP_SAVE=1
      shift
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: scripts/agent_verify.sh [--with-smoke] [--keep-save]"
      exit 2
      ;;
  esac
done

echo "[agent-verify] Running cargo test..."
cargo test

if [[ "$WITH_SMOKE" -eq 1 ]]; then
  echo "[agent-verify] Running TUI smoke flow..."
  args=(--no-build)
  if [[ "$KEEP_SAVE" -eq 1 ]]; then
    args+=(--keep-save)
  fi
  scripts/agent_tui_smoke.sh "${args[@]}"
fi

echo "[agent-verify] Done."
