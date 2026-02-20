# TUI Visual System (Color, Icons, Animation)

> Scope: modern terminals with UTF-8, full color, and icon-capable fonts.
> This doc defines the visual architecture so richer presentation stays stable
> and testable.

---

## Goals

- Make the TUI feel expressive and readable, not plain text-only.
- Keep rendering deterministic and bounded in performance.
- Provide graceful fallbacks for weaker terminal capabilities.
- Keep gameplay logic independent from visual effects.

## Non-Goals

- Pixel-graphics rendering in terminal.
- Vendor-specific image protocols as a hard dependency.
- Unbounded high-FPS effects that hurt input responsiveness.

---

## Capability Tiers

All visual features must map to one of these runtime tiers.

| Tier | Terminal capability | Visual policy |
|---|---|---|
| `T0` | ASCII only | text + ASCII borders, no icons |
| `T1` | UTF-8 glyphs | Unicode symbols/icons, limited color |
| `T2` | 256 colors | semantic palette + icon usage |
| `T3` | Truecolor | full palette, gradients, richer animation cues |

Renderer startup should detect and cache the tier in UI state. Effects must use
tier-aware fallbacks instead of failing.

Implementation note:
- current runtime detection + token wiring live in `src/ui/tui/theme.rs`.

---

## Color System

Use semantic tokens, not hard-coded colors in screen code.

- `surface_bg`, `panel_bg`, `panel_border`
- `text_primary`, `text_muted`, `text_emphasis`
- `accent_primary`, `accent_secondary`
- `state_info`, `state_success`, `state_warning`, `state_danger`
- `combat_hit`, `combat_crit`, `combat_miss`

Rules:

- Each screen reads tokens from a shared theme struct.
- Add a high-contrast theme variant for accessibility.
- Never encode game state semantics with color alone; keep text labels.

---

## Icon Policy

Icons should be semantic and optional.

- Define an icon atlas mapping symbolic IDs to glyphs.
- Keep per-tier fallback (`T0` ASCII string, `T1+` Unicode/icon glyph).
- Avoid icon-only controls; always pair with text labels.

Suggested first set:

- Health/heart, mana/star, quest/scroll, faction/banner, danger/skull
- Input hints (`[Enter]`, `[Esc]`) remain text-first for portability

---

## Animation Model

Animation must be event-driven and time-bounded.

- Use tick-based updates from the app loop (`game-loop.md`).
- Keep effect state in UI layer only; gameplay state remains pure.
- Prefer short transitions (100-400ms) and low-frequency ambient motion.
- Support a reduced-motion mode that disables non-essential effects.

Initial effect primitives:

- Pulse (selection, important status)
- Flash (damage/heal/combat outcome)
- Sweep/Wipe (screen transitions)
- Float-up text (damage numbers, status tags)
- Cursor shimmer (active tile/target)

---

## Performance Budget

- Prioritize input latency over animation smoothness.
- Target baseline:
  - active animation bursts: 15-30 FPS equivalent
  - ambient/background motion: 8-15 FPS equivalent
- Avoid full-screen redraw-heavy effects each tick when a smaller dirty region
  can be updated.

If frame time exceeds budget, degrade in this order:
1. disable ambient effects
2. reduce transition frequency
3. downgrade from `T3` to `T2` style rules at runtime

---

## Testing and Validation

- Add deterministic animation tests for state transitions where practical.
- Extend agent smoke checks with representative visual-state transitions.
- Add manual verification checklist:
  - tier detection
  - icon fallback correctness
  - reduced-motion behavior
  - readability in high-contrast mode

Related test entry point: [../testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md)

---

## Milestone Integration

- Milestone 8: establish foundations (capability profile + theme plumbing hooks).
- Milestone 12: deliver full visual pass (palette, icon atlas, animation layer,
  accessibility toggles, and docs/README updates).

Task source of truth: [../tasks/backlog.md](../tasks/backlog.md)

Verification helpers:
- `scripts/profile_startup.sh`
- `scripts/release_check.sh`

---

## Related Docs

- [ui-layer.md](ui-layer.md)
- [game-loop.md](game-loop.md)
- [../gameplay/overview.md](../gameplay/overview.md)
- [../DOCS_MAP.md](../DOCS_MAP.md)
