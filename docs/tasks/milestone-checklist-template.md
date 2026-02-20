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
- [ ] `scripts/agent_tui_smoke.sh --no-build`
- [ ] Soak run (when applicable): `scripts/agent_tui_smoke.sh --soak --profile standard --minutes <n> --token-efficient`

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
