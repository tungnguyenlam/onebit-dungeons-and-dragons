#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

printf 'Running authored content validation...\n'
cargo test load_all_authored_regions -- --nocapture
cargo test load_all_authored_quests -- --nocapture
printf 'Content validation passed.\n'
