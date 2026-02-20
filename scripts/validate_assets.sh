#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[validate-assets] Running asset graph validation..."
cargo run --quiet -- --validate-assets
echo "[validate-assets] Done."
