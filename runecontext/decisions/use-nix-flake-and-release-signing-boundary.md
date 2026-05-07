---
schema_version: 1
id: use-nix-flake-and-release-signing-boundary
title: Use Nix Flake And Release Signing Boundary
originating_changes:
  - CHG-2026-017-50a5-set-up-nix-local-development-and-release-artifacts
related_changes: []
---

# Use Nix Flake And Release Signing Boundary

## Decision

Use a Nix flake for the local development shell and future release artifact generation. Keep `flake.nix` small and place reusable Nix logic in a root-level `nix/` directory. Nix should produce unsigned artifacts. GitHub release automation should sign and publish artifacts later.

## Rationale

A Nix flake gives the project a pinned, reproducible development foundation before implementation starts. A root-level `nix/` directory keeps dev shell, checks, packages, and release metadata reviewable as separate modules instead of concentrating build logic in `flake.nix`. Keeping signing outside the Nix artifact build separates deterministic build output from release authority, secrets, and publication policy.

## Consequences

- The Nix flake should be implemented before other MVP engineering work.
- nixpkgs unstable is acceptable when it best supports the required core, diagnostic, packaging, and later selected frontend dependencies.
- The initial Nix shell should avoid locking in Flutter, Dart, Qt, Tauri, Electron, or any production frontend framework before frontend evaluation.
- Nix logic should be factored into `nix/dev-shell.nix`, `nix/checks.nix`, `nix/packages/`, and eventually `nix/release/metadata.nix`.
- Release outputs should be designed as unsigned until a release workflow signs them.
- Unsigned release artifact package outputs should be shaped before GitHub release automation is added.
- Signing credentials must remain in GitHub release infrastructure or another approved secret store, never in the repository.
- Later release workflow work can build on the same artifact boundary instead of inventing a separate packaging path.
