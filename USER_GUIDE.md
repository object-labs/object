# Object User Guide

This guide is downstream from Keel and documents what Object is meant to do for operators and downstream users.

## Product Story

Object is a project to make running blockchain/exchange infrastructure economically viable for independent operators. By building on Transit, it targets lower and more predictable node operating costs while enabling exchange-style throughput, EVM compatibility, and cross-market execution across both spot and perpetual instruments.
The production model is an EVM chain on the Transit substrate, with native bridge transfer as the external value layer.

It is for:
- Operators evaluating node infrastructure costs,
- Protocol teams needing exchange-class data-plane performance,
- Users needing interoperable chain execution, high-frequency order handling, and query access in a coherent stack.

Object is intended to perform better than generic alternatives by coupling execution, settlement, and data exposure around explicit operator efficiency and native chain-to-chain value movement.

## Core User Flows

1. **Repository bootstrap**
   - Clone repository.
   - Run `nix develop`.
   - Run `just build` to verify initial compile.
2. **Iteration and validation**
   - Make scoped changes in `src/` or supporting files.
   - Run `just check`, `just fmt`, and `just clippy`.
   - Run `just test` to exercise available test surfaces.
3. **Exchange validation path (roadmap)**
   - Submit test orders via exchange gateway or mocked client.
   - Route spot and perp order intents through separate market handlers.
   - Confirm finality, settlement proof emission, and projection updates.
4. **Failure and recovery**
   - If verification fails, isolate to the smallest file or command change.
   - Fix and re-run checks in sequence until the local surface is healthy.

## Exchange User Model

Object is designed around two market primitives:

- **Spot**: deterministic spot order execution and immediate settlement against balances.
- **Perpetuals**: position-based trading with explicit funding/risk state transitions.

Native bridging is the value transport layer:

- Deposits and withdrawals flow through bridge attestations and finalized burn/mint proofs.
- All external asset movement is intended to be mediated through bridge-native streams, not proprietary custodial settlement.

## Personas

- **Node Operator**: evaluates, runs, and operates the stack.
- **Market Operator**: tunes order, matching, and risk assumptions.
- **Protocol Engineer**: implements execution, bridge, and query paths.
- **Stakeholder**: tracks planning state, milestones, and tradeoffs.

## Acceptance Lens

For a change to be considered acceptable from the user perspective:
- it is reproducible in `nix develop`,
- it does not invalidate existing local workflows,
- it improves or preserves economic-operational clarity for operators,
- and claims in docs match actual runtime behavior.

Current implementation is scaffolding, so most user-facing functions are roadmap-level until code catches up. That distinction should be explicit in every feature update.
