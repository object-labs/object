# AGENTS.md

Shared guidance for AI agents working with Object.

## Downstream Contract

This repository uses Keel as its project-management engine. This file is downstream from Keel and should remain recognizable when upstream engine guidance changes.

`AGENTS.md` and `INSTRUCTIONS.md` are the sync-sensitive files in this scaffold. When you absorb a newer Keel version, preserve the `PROJECT-SPECIFIC` blocks instead of rewriting this file from memory.

## Read This First

1. `INSTRUCTIONS.md` for the repo's procedural loop.
2. `POLICY.md` for local operational invariants.
3. `ARCHITECTURE.md`, `PROTOCOL.md`, and `USER_GUIDE.md` for system context and product behavior.
4. `CODE_WALKTHROUGH.md` for source layout and execution flow.
5. `keel mission next --status` (or equivalent Keel board status command) for live planning state.

## Core Principles

- Use Keel as the canonical planning and lifecycle surface.
- Keep technical claims grounded in code and measurable runtime outputs.
- Prefer reproducible automation over manual state tracking.
- Resolve decisions against the explicit contracts in foundational documents first.
- Escalate any non-local risk (security, legal, economics, customer-facing behavior) for human review.

## Decision Resolution Hierarchy

When faced with ambiguity, resolve decisions in this order:
1. **ADRs**: Binding architectural constraints.
2. **CONSTITUTION**: Collaboration and release philosophy.
3. **POLICY**: Operational invariants.
4. **ARCHITECTURE**: Source boundaries and implementation constraints.
5. **PLANNING artifacts**: PRD/SRS/SDD for the active mission.

## Foundational Documents

- `INSTRUCTIONS.md` — procedural workflow for repo operations.
- `POLICY.md` — local invariants and release gates.
- `CONSTITUTION.md` — collaboration philosophy.
- `ARCHITECTURE.md` — runtime architecture and technical boundaries.
- `PROTOCOL.md` — external coordination and handoff contract.
- `CODE_WALKTHROUGH.md` — source layout and local code path orientation.
- `USER_GUIDE.md` — operator-facing product story and expected behavior.
- `.keel/adrs/` — binding architecture decisions, once introduced.

Use this order when interpreting constraints: ADRs → Constitution → Policy → Architecture → Planning artifacts.

## Project-Specific Conventions

<!-- BEGIN PROJECT-SPECIFIC -->
- Object is a Rust-first experimental scaffold for building an economically viable, operator-first blockchain stack.
- Use `.direnv`/`nix develop` as the primary local environment entrypoint.
- Use `just` for reproducible local commands: `build`, `check`, `fmt`, `clippy`, `test`, and `run`.
- Keep all README/contract updates synchronized with actual repo state; the codebase is intentionally young and claims should not outpace implementation.
- Treat board state transitions and claims of completion as proof-bearing; close only when code, docs, and verification are aligned.
<!-- END PROJECT-SPECIFIC -->

## Sync Notes

- Upstream source: Keel's `AGENTS.md`.
- Preserve the `PROJECT-SPECIFIC` block during syncs.
- Keep procedural details in `INSTRUCTIONS.md`, not here.
