# Worktree Triage (2026-02-26)

Safety branch: `worktree-triage`
Snapshot: `/tmp/worktree_triage/status_short.txt`
Detailed report: `/tmp/worktree_triage/triage_report.txt`

## Counts
- Total changed paths: 164
- Tracked modified: 131
- Untracked: 33
- After split commits: 111 remaining (`89` tracked, `22` untracked)

## Commits created during triage
- `4ed89a8` — isolate M59/M60 implementation + docs
- `d79ff62` — isolate M57/M58 crafting + bestiary/lore screens

## Buckets
- `runtime_noise` (2): `save.toml`, `amp`
- `docs` (11): README/gameplay/tasks milestone docs
- `milestone_core` (4): weather/ending/new boss core files
- `legacy_probable` (6): prior crafting/bestiary/underdark additions
- `unknown_review` (141): broad app/data/game/ui changes needing split review

## Cleanup Sequence (non-destructive)
1. Commit runtime-ignore policy decisions first (if desired).
2. Commit milestone-core + docs as one or two focused commits.
3. Move `legacy_probable` into its own commit (if intended work).
4. Review `unknown_review` in small chunks by subsystem (`src/app`, `src/game`, `assets/`).
5. For anything accidental, revert only after explicit confirmation.

## Suggested next commands
```bash
# inspect unknown chunk by chunk
git diff -- src/app
git diff -- src/game
git diff -- assets

# prepare focused staging
git add src/game/world/weather.rs src/game/story/ending.rs src/ui/tui/screens/ending.rs assets/monsters/void_architect.toml

# add milestone docs
git add docs/tasks/current-sprint.md docs/tasks/backlog.md docs/tasks/done.md docs/tasks/milestones/m59.md docs/tasks/milestones/m60.md
```
