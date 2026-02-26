# Milestone Completion Checklist Template

Use this at handoff when closing a milestone.

## Milestone
- Name:
- Date:
- Owner:

## Implementation
- [ ] Scope items implemented
- [ ] Non-goals preserved
- [ ] Risk mitigations applied

## Verification
- [ ] `cargo test`
- [ ] `cargo run -- --validate-assets`
- [ ] `python3 scripts/visual_check.py --scenario enter_world --artifact none --show`
- [ ] Soak run (when applicable): `for i in $(seq 1 <n>); do python3 scripts/visual_check.py --scenario enter_world --artifact none; done`

## Artifacts
- [ ] Updated docs under `docs/`
- [ ] Updated `docs/tasks/backlog.md`
- [ ] Updated `docs/tasks/current-sprint.md` handoff
- [ ] Updated `docs/tasks/done.md`

## Reproduction Commands
- Validation:
- Smoke:
- Soak:

## Notes for Next Agent
- Remaining risks:
- Follow-up task:
