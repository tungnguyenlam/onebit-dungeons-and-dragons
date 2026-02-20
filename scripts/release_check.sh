#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Running release readiness checks..."
cargo test
scripts/validate_content.sh
scripts/profile_startup.sh
echo "Release readiness checks passed."
