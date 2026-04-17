# CLAUDE.md

Guidance for **Claude Code** when working with this repository.

## Shared Contract

Before doing work, read:
1. `AGENTS.md`
2. `INSTRUCTIONS.md`
3. `POLICY.md`
4. `ARCHITECTURE.md`
5. `PROTOCOL.md`

Those files are the repo-wide operating contract.

## Project-Specific Claude Notes

<!-- BEGIN PROJECT-SPECIFIC -->
- Use `nix develop` before code execution or verification.
- Keep edits minimal and reproducible in this bootstrap repo; prefer explicit docs updates for scope changes.
- Favor `just` targets that already exist in the repository.
- Treat README and foundational docs as part of the deliverable for major behavior changes.
<!-- END PROJECT-SPECIFIC -->
