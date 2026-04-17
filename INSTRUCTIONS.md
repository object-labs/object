# INSTRUCTIONS.md

Procedural instructions for humans and agents working with Object through Keel.

## Downstream Contract

This file is downstream from Keel and preserves local operating details while keeping the upstream turn-loop recognizable.

Mission Stack coordination is part of the formal protocol surface. Hydrate `PROTOCOL.md` instead of inventing cross-repo coordination in chat or commit lore.

When syncing from a newer Keel version, preserve the `PROJECT-SPECIFIC` block instead of re-authoring local operating context from scratch.

## The Turn Loop

Use Keel’s canonical loop as the baseline:

1. **Orient**: check board status and mission context.
2. **Inspect**: open the active mission or role lane.
3. **Pull**: claim one bounded task.
4. **Ship**: implement and produce local proof.
5. **Close**: record completion and align planning state with repository changes.

## Primary Workflows

### Operator (Implementation)
- **Context**: identify local role and active board item.
- **Action**: implement scoped changes and attach proof to the relevant story/mission.
- **Constraint**: every acceptance claim must map to code changes and verification evidence.

### Manager (Planning)
- **Context**: review active missions, lanes, and acceptance needs.
- **Action**: write and refine planning artifacts and keep downstream sequencing explicit.
- **Constraint**: avoid starting implementation where planning invariants are unclear.

### Explorer (Research)
- **Context**: unresolved technical assumptions and implementation risk.
- **Action**: validate choices, capture findings, and return decision-ready notes.
- **Constraint**: only graduate to planned work when assumptions are resolved.

## Repo-Specific Turn Surfaces

<!-- BEGIN PROJECT-SPECIFIC -->
- Run all local validation through:
  - `nix develop`
  - `just build`
  - `just check`
  - `just fmt`
  - `just clippy`
  - `just test`
- Required local checks for code changes:
  - Formatting and linting must pass.
  - Test command should run clean for implemented test surfaces.
  - Update docs whenever command surface or behavior changes.
- This repo is in an early bootstrap phase; mark roadmap claims clearly and avoid treating placeholders as implemented behavior.
<!-- END PROJECT-SPECIFIC -->

## Hygiene Rules

- Use Keel as the canonical planning surface.
- Prefer board-backed proof over memory or chat summaries.
- Treat mission/state transitions as part of the implementation definition of done.
- Keep foundational docs updated whenever architecture or operating assumptions change.
