# Agent Smoke Testing

Use this guide for deterministic, non-interactive smoke checks.

Primary tools:
- `python3 scripts/visual_check.py`
- `scripts/agent_verify.sh` (tests + asset validation)

---

## What It Tests

The scenario runner drives the real game in text mode and validates key gameplay flow (menu -> world entry -> interactions) with reproducible key sequences from `tests/visual_scenarios.json`.

---

## Usage

From repo root:

```bash
# Standard regression entry point
scripts/agent_verify.sh

# List available visual scenarios
python3 scripts/visual_check.py -l

# Smoke check using a predefined scenario
python3 scripts/visual_check.py --scenario enter_world --artifact none --show

# Save a compact final-state artifact
python3 scripts/visual_check.py --scenario enter_world
```

### Artifact and Debug Options

```bash
# Full step history
python3 scripts/visual_check.py --scenario enter_world --verbose-steps --artifact full --history

# Custom key sequence smoke
python3 scripts/visual_check.py "jjl\r" --name custom_smoke --show
```

---

## Soak-Style Run (Replacement for old `--soak`)

Use repeated scenario execution for a timed soak run:

```bash
end=$(( $(date +%s) + 300 )) # 5 minutes
while [ "$(date +%s)" -lt "$end" ]; do
  python3 scripts/visual_check.py --scenario enter_world --artifact none >/dev/null
  python3 scripts/visual_check.py --scenario combat_flee --artifact none >/dev/null
done
```

This preserves deterministic behavior with the scenario runner.

---

## Notes

- `visual_check.py` is the active smoke/soak runner.
- `visual_check.py` does not require `expect` or a TTY.
- Scenario list and key sequences live in `tests/visual_scenarios.json`.

See also: [step-through-testing.md](step-through-testing.md)
