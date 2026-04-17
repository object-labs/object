# Object Constitution

This document is downstream from Keel and defines how Object coordinates work and tradeoffs in this repository.

## Why This Exists

Object is an operator-first infrastructure project. The project only succeeds if node operators can run reliable, high-throughput infrastructure without being forced into systemic losses. This document protects that goal by forcing delivery to remain economically rational, testable, and transparent.

## Decision Hierarchy

When ambiguity appears, resolve it using this order:
1. ADRs (once introduced)
2. This Constitution
3. POLICY
4. ARCHITECTURE
5. Active planning artifacts (PRD/SRS/SDD)
6. Explicit human judgment for non-technical risk

## Project Values

- Operator viability is the primary product constraint.
- Economic efficiency should improve as features are added, or be explicitly justified.
- New runtime claims require reproducible evidence.
- Safety and predictability beat novelty in execution and sequencing.
- Cross-agent collaboration should be low-friction: clear tasks, explicit proof, and minimal stale context.

## Collaboration Rules

- Route governance and planning through the Keel surface, not ad hoc local conventions.
- Avoid changing planning state without a clear link to implementation or documentation changes.
- Prefer small, reversible delivery increments over speculative rewrites.
- When technical risk is low but strategic risk is high, pause and require human sign-off.

## Good Enough to Ship

- Build path is stable for the current scope.
- Local verification in this repo runs clean:
  - `nix develop` environment works.
  - `just build`, `just check`, `just fmt`, and `just clippy` are aligned.
  - `just test` is current for the available test surface.
- Documentation reflects current behavior and does not claim future systems as shipped.
- Board state is consistent with repository changes.

## Revision Notes

Keep this short and durable. Place tactical process changes in `INSTRUCTIONS.md` and technical constraints in `POLICY.md` or `ARCHITECTURE.md`.
