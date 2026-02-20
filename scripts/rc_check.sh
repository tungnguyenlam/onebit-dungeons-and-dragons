#!/usr/bin/env bash
# scripts/rc_check.sh — M24: Release Candidate gate script
#
# Runs all checks required before an RC tag can be cut.
# Exit code is 0 only if every tier passes.
#
# Usage:
#   ./scripts/rc_check.sh           # all tiers
#   ./scripts/rc_check.sh --fast    # tier-1 only (< 30 s)
#   ./scripts/rc_check.sh --slow    # tier-1 + tier-2 (compiles + full tests)
#
# Tiers:
#   T1 (fast)  : format, lint, asset validate, save-suite
#   T2 (slow)  : full cargo test, TUI smoke
#   T3 (rc)    : T1 + T2 + soak profile

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

TIER="${1:---all}"
PASS=0
FAIL=0
SKIPPED=0

step() {
    local tier="$1"
    local name="$2"
    shift 2
    echo -e "${BLUE}[${tier}]${NC} ${name}..."
    if "$@"; then
        echo -e "  ${GREEN}✓ pass${NC}"
        PASS=$((PASS + 1))
    else
        echo -e "  ${RED}✗ FAIL${NC}"
        FAIL=$((FAIL + 1))
    fi
}

skip() {
    local name="$1"
    echo -e "${YELLOW}[skip]${NC} ${name}"
    SKIPPED=$((SKIPPED + 1))
}

cd "$REPO_ROOT"

# ---------------------------------------------------------------------------
# Tier 1 — fast (~10-30 s)
# ---------------------------------------------------------------------------
echo -e "\n${BLUE}=== Tier 1: Format / Lint / Assets ===${NC}"

step T1 "cargo fmt --check"        cargo fmt --check
step T1 "cargo clippy (no errors)" cargo clippy -- -D warnings
step T1 "cargo build"              cargo build
step T1 "asset validation"         cargo run -- --validate-assets
step T1 "save/load roundtrip suite" \
    cargo test save -- --nocapture
step T1 "region traversal suite" \
    cargo test validate -- --nocapture
step T1 "quest runtime suite" \
    cargo test quest -- --nocapture
step T1 "combat AI suite" \
    cargo test "game::combat::ai" -- --nocapture

# ---------------------------------------------------------------------------
# Tier 2 — slow (full test matrix + smoke)
# ---------------------------------------------------------------------------
if [[ "$TIER" == "--fast" ]]; then
    skip "Tier 2: full cargo test (skipped — --fast mode)"
    skip "Tier 2: TUI smoke (skipped — --fast mode)"
else
    echo -e "\n${BLUE}=== Tier 2: Full Test Suite ===${NC}"
    step T2 "cargo test (all)"  cargo test
    if command -v expect >/dev/null 2>&1; then
        step T2 "TUI smoke" \
            bash "$SCRIPT_DIR/agent_tui_smoke.sh" --no-build
    else
        skip "TUI smoke (expect not installed)"
    fi
fi

# ---------------------------------------------------------------------------
# Tier 3 — RC gate (soak profile)
# ---------------------------------------------------------------------------
if [[ "$TIER" == "--all" ]]; then
    echo -e "\n${BLUE}=== Tier 3: RC Soak Profile ===${NC}"
    if command -v expect >/dev/null 2>&1; then
        step T3 "soak (1 min)" \
            bash "$SCRIPT_DIR/agent_tui_smoke.sh" \
                --soak --profile standard --minutes 1 --token-efficient --no-build
    else
        skip "soak profile (expect not installed)"
    fi
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo -e "${BLUE}=== RC Check Summary ===${NC}"
echo -e "  ${GREEN}Pass:${NC}    $PASS"
echo -e "  ${RED}Fail:${NC}    $FAIL"
echo -e "  ${YELLOW}Skipped:${NC} $SKIPPED"

if [[ $FAIL -gt 0 ]]; then
    echo -e "\n${RED}RC check FAILED — $FAIL step(s) did not pass.${NC}"
    exit 1
else
    echo -e "\n${GREEN}RC check passed — ready to cut release tag.${NC}"
    exit 0
fi
