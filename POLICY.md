# Object Policy

This document is downstream from Keel and specifies the operational invariants that should remain stable for this repository.

## Engine Contract

Object uses Keel for lifecycle tracking and mission sequencing.

- Board state lives under `.keel/`.
- Board state transitions should be made through Keel commands, not manual edits to planning files.
- `.keel/cache/` and similar generated artifacts are tooling outputs and may be regenerated.

## Core Objective: Low Drift

The objective is to keep repository state, board state, and implementation claims aligned.

- No change should claim completion unless local command checks and docs reflect that change.
- Do not leave placeholders in docs for active planning-critical sections.
- Do not rename or remove foundational files (`CONSTITUTION.md`, `POLICY.md`, `ARCHITECTURE.md`, `PROTOCOL.md`, etc.) without updating references.

## Repo Invariants

Before merging any runtime-affecting change, satisfy at least:
- `just fmt` (or equivalent formatting command for changed code).
- `just check`.
- `just clippy`.
- `just test` where practical for available test targets.

For purely procedural updates (documentation, planning notes, scaffolding metadata), document any missing checks and rationale.

## Evidence and Escalation

- Runtime and behavior changes require evidence that can be reproduced locally.
- Economic-claim statements (cost, throughput, operator margin, etc.) require either code-path confirmation or explicit "not yet implemented" labeling.
- Escalate when a change touches:
  - production-facing behavior,
  - security-critical dependency boundaries,
  - externally visible economic or settlement guarantees.

## External Interaction Boundaries

- Cross-repo planning and execution should remain within the formal Mission Stack model.
- This repository should not unilaterally mutate another repository’s `.keel/` state.
- Any future managed-worktree or checkpoint expectations should be codified here.

## Safety Rails

- Never commit or document secrets.
- Keep generated artifacts and build outputs out of version intent unless explicitly tracked.
- Do not broaden scope in implementation without updating corresponding architecture/product documentation.
