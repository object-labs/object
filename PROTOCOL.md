# Object Protocol

This document is downstream from Keel and defines the protocol-level expectations around coordination and handoff for Object.

## Downstream Contract

This repository inherits Keel as the upstream coordination engine. In this repo, this file captures the stable contract that other systems and contributors can rely on when interacting with Object’s work state and handoff expectations.

## Scope

Object protocol surfaces are focused on:
- ingesting requests from humans, agents, and linked repos through repository workflow,
- preserving source-of-truth in `.keel/`,
- emitting clear completion receipts via the same board mechanism.

## Mission Stack Coordination

- This repo treats board state as authoritative for planning and sequencing.
- Work in other repositories should not directly mutate Object planning artifacts.
- Cross-repo work should pass through explicit Mission Stack communication and branch conventions documented for each active stack.
- Incomplete or speculative work is handled as planning artifacts until it can be implemented and verified in code.

## External Ingress

1. Request arrives as human/agent directive in the local workflow context.
2. Directive is represented in the relevant Keel artifact.
3. Work is claimed and executed by role in turn loop.
4. Completion requires local proof and board transition.

## Data Contracts

No external runtime API contracts are stable yet. The current contracts are planning contracts:
- mission/epic/voyage/story identifiers and status fields in Keel entities,
- file-based proof and state transitions,
- commit references used as completion receipts.

## Local Exceptions

- Until execution APIs are defined, no public JSON/API contract is guaranteed.
- Runtime behavior claims in product-facing docs must explicitly match implemented code.
- Any future external-facing protocol contracts should be added here before external clients rely on them.
