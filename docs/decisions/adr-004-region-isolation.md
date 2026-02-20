# ADR-004 — Region Isolation (No Single World Map)

**Date:** 2026-02-20  
**Status:** Accepted

## Decision

The game world is divided into **independent regions**, each in its own folder
under `assets/regions/<slug>/`. There is no single large world map file.
The player travels between regions via explicit travel nodes; loading a new
region discards the previous region's data from memory.

## Rationale

- **Context safety for agents:** An AI agent authoring or modifying a region
  reads only that region's folder. It never needs to load a monolithic world
  file, preventing context overflow and irrelevant file jumping.
- **Memory efficiency:** Only one region's rooms, NPCs and dialog are in memory
  at a time.
- **Parallel development:** Two agents can work on different regions
  simultaneously without file conflicts.
- **Scope control:** Each region is a bounded, testable unit. A region can be
  completed and marked done independently of others.

## Rejected Alternatives

- **Single `overworld.toml` with all rooms:** Simple for small games, but
  grows without bound and causes context overflow when an agent needs to
  read it as a whole.
- **Chunked big map (scroll):** More complex renderer, more complex FOV, no
  meaningful agent isolation benefit.

## Consequences

- Travel between regions is a discrete event (a loading screen flash or
  transition message), not seamless scrolling.
- Cross-region references (e.g. a quest spanning two regions) use slug strings
  (`"emberpeak-summit:south_slope"`) resolved at runtime — not direct object
  references.
- The region index at `docs/content/regions/index.md` is the only file that
  must reference all regions — an agent modifying a *single* region never
  needs to read the index.
