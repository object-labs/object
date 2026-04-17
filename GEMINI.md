# GEMINI.md

Guidance for **Gemini CLI** and **Google AI Studio** when working with this repository.

## Shared Contract

Before doing work, read:
1. `AGENTS.md`
2. `INSTRUCTIONS.md`
3. `POLICY.md`
4. `ARCHITECTURE.md`
5. `PROTOCOL.md`

Those files are the repo-wide operating contract.

## Project-Specific Gemini Notes

<!-- BEGIN PROJECT-SPECIFIC -->
- Source of truth for command workflows is `justfile`.
- Current codebase is a bootstrap; avoid assumptions about runtime features not yet implemented.
- Keep recommendations in sync with README and foundational docs; avoid proposing functionality that requires unimplemented infrastructure.
- Use Nix environment assumptions from `flake.nix` before suggesting tooling changes.
<!-- END PROJECT-SPECIFIC -->
