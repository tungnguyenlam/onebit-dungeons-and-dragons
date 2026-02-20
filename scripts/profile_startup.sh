#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Profiling data/startup-path load (test harness)..."
/usr/bin/time -p cargo test profile_asset_load_smoke -- --nocapture
