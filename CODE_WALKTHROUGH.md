# Object Code Walkthrough

This document orients contributors and agents to the source layout, abstractions, and data flow in the current Object scaffold.

For governance philosophy see [CONSTITUTION.md](CONSTITUTION.md); for architecture see [ARCHITECTURE.md](ARCHITECTURE.md); for coordination protocol see [PROTOCOL.md](PROTOCOL.md).

## Repository Layout

- `src/main.rs`: Rust crate entrypoint.
- `Cargo.toml`: crate metadata.
- `flake.nix`: Nix shell/package definitions.
- `justfile`: local command aliases.
- `rust-toolchain.toml`: pinned Rust toolchain configuration.
- `.keel/`: planning and mission files.
- `AGENTS.md`, `INSTRUCTIONS.md`, `POLICY.md`, `ARCHITECTURE.md`, `PROTOCOL.md`, `USER_GUIDE.md`, `CONSTITUTION.md`: foundational docs.

## Key Abstractions

At bootstrap, there are no domain abstractions yet beyond the Rust `main` entrypoint. Future modules should evolve around a Transit-style exchange design:
- spot execution and settlement,
- perpetual execution and risk accounting,
- bridge-based liquidity movement,
- query/analytics paths,
- operator-facing interfaces.

## State and Lifecycle

Current runtime state is minimal: startup prints from `main`.

Planned state concerns are not yet implemented and should be introduced intentionally with bounded modules and explicit tests for:
- spot matching,
- perpetual state updates,
- bridge settlement proofs,
- high-frequency order replay behavior.

## Command / Request Flow

Current flow:
1. User runs `cargo run` (via `just run`).
2. Binary executes `main`.
3. Output is emitted to stdout (currently `"Hello, world!"`).

As the system grows, this section should describe request normalization, execution routing, and persistence boundaries.

## Configuration

- `flake.nix`: source of reproducible Rust and system tooling.
- `rust-toolchain.toml`: compiler and toolchain pinning.
- `justfile`: verification and development commands.
- `.keel/` and board metadata: planning configuration and sequencing.

## Where to Look

| I want to... | Start here |
| --- | --- |
| Update foundational docs | The matching foundational markdown file |
| Change local developer commands | `justfile` |
| Adjust development environment | `flake.nix` |
| Add runtime functionality | `src/main.rs` (then split modules as scope grows) |
| Adjust planning governance | `keel.toml` and `.keel/` |
