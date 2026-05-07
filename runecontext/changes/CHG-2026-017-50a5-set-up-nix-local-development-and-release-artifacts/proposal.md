## Summary
Set up Nix local development and release artifacts.

## Problem
KORCompanion needs a reproducible local development environment before implementation work begins. The project should also establish its release direction early so build outputs are deterministic, reviewable, and compatible with a later GitHub release workflow.

Without this foundation, contributors may use inconsistent core, packaging, diagnostic, and system-library versions, and release packaging decisions may be bolted on after application architecture has already assumed local-only build behavior.

## Proposed Change
Add a Nix flake as the first MVP implementation change.

The change should provide:

- A `flake.nix` for a reproducible local development shell.
- A pinned `flake.lock`.
- A root-level `nix/` directory for reusable Nix modules instead of putting all Nix logic in `flake.nix`.
- A dev shell with core development, testing, formatting, linting, packaging-support, and host/device diagnostic tools where practical.
- Common local commands documented for entering the shell, fetching dependencies, running tests, formatting, linting, and validating headless workflows.
- Initial Nix package/check structure that can later produce unsigned project release artifacts without reorganizing the flake.
- A future GitHub release workflow direction where unsigned artifacts are signed and published as release assets.

This change should not attempt to fully implement production release signing in the first step. It should establish the structure and conventions that later release automation will use.

## Why Now
This should be the first MVP implementation item because every later change depends on a consistent development shell and predictable build toolchain.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Headless foundation and risky host/device workflow validation are the initial implementation lane.
- Linux development should be well-supported first, while the flake should not prevent macOS development where practical.
- Using nixpkgs unstable is acceptable for this project when it provides the core, packaging, or diagnostic dependencies needed for current development.
- Frontend-specific tooling such as Flutter/Dart, Qt, Tauri, or Electron should be added only after frontend evaluation selects it.
- Release artifacts should be unsigned at Nix build time, then signed in the release workflow boundary.

## Out of Scope
- Full GitHub release workflow implementation.
- Secret management for release signing keys.
- Publishing release assets.
- Final installer, notarization, or platform-signing implementation.
- Mobile build support.
- Store, notarization, or platform-specific app signing.
- Device-specific KOReader installation behavior.

## Impact
The project gains a reproducible starting point for local development and a clear release-build boundary that can evolve into signed GitHub releases without reworking the build model.
