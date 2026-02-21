#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# agent_verify.sh - Core verification script for OneBit D&D
# Standardizes checks to ensure the codebase is stable.

echo "[agent-verify] Running all tests (including functional smoke flows)..."
cargo test

echo "[agent-verify] Validating asset cross-references..."
cargo run --quiet -- --validate-assets

echo "[agent-verify] Done."
