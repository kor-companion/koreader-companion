---
schema_version: 1
id: development-and-release
title: Development And Release
originating_changes:
  - CHG-2026-017-50a5-set-up-nix-local-development-and-release-artifacts
revised_by_changes:
  - CHG-2026-018-d4f1-finalize-github-release-workflow-and-install-documentation
---

# Development And Release

The project should use a Nix flake as the local development and future release-artifact foundation.

## Local Development

The first development environment target is a reproducible dev shell.

Requirements:

- `nix develop` enters the standard contributor environment.
- Rust core development, test, formatting, linting, and packaging-support tools are available inside the shell.
- Host/device diagnostic prerequisites are documented where Nix cannot fully abstract them.
- Common development commands are documented.
- Dependency versions are pinned by `flake.lock`.
- nixpkgs unstable is acceptable when it is the practical source for current core, packaging, or diagnostic dependencies.
- Host prerequisites that cannot be fully managed by Nix are documented.
- Flutter, Dart, Qt, Tauri, Electron, or other frontend-specific dependencies should be added only after the frontend evaluation selects them.
- Casual contributors should have a documented non-Nix path where practical, such as standard Rust toolchain commands, while Nix remains canonical for CI and release artifacts.

Expected Rust tooling:

- Rust toolchain for the headless core.
- `cargo` commands for build, test, format, lint, and documentation generation.
- SQLite development dependencies where needed by the selected Rust SQLite binding.
- Cross-platform packaging helpers only when they are needed by implemented artifacts.

The dev shell should be introduced before application implementation so every later MVP change can rely on the same environment.

## Nix Layout

Nix logic should be split out of `flake.nix` into a root-level `nix/` directory.

Expected layout:

- `flake.nix` wires inputs, systems, formatter, dev shell, checks, and packages.
- `flake.lock` pins inputs.
- `nix/dev-shell.nix` defines the local contributor environment.
- `nix/checks.nix` defines flake checks.
- `nix/packages/` defines package and unsigned artifact outputs.
- `nix/release/metadata.nix` centralizes release metadata when artifact outputs need stable names or versions.

## Release Direction

The project should eventually build release artifacts through Nix first, then sign and publish them through GitHub release automation.

Expected boundary:

- Nix produces unsigned project artifacts.
- Unsigned artifacts are named and treated as unsigned.
- Unsigned artifacts are exposed through flake package outputs so GitHub Actions can build them without duplicating packaging logic.
- Release metadata and artifact naming live in Nix files, not only in GitHub workflow YAML.
- The GitHub release workflow signs artifacts after generation.
- The GitHub release workflow publishes signed artifacts and any required checksums or manifests.
- Signing keys, certificates, tokens, and release credentials are never committed to the repository.

## GitHub Release Workflow

The final MVP release workflow should complete the release path prepared by the Nix artifact outputs.

Requirements:

- The workflow builds unsigned artifacts from Nix flake package outputs.
- The workflow signs artifacts after Nix generation.
- The workflow publishes signed artifacts to GitHub Releases.
- The workflow publishes checksums or a release manifest with the assets.
- Production releases fail closed when signing material is unavailable.
- Packaging logic stays in Nix; GitHub Actions orchestrates, signs, and publishes.
- Secret names and maintainer setup are documented without committing secret values.

## README Installation Documentation

`README.md` should include an installation section for end users.

Requirements:

- Link users to GitHub Releases.
- Explain which asset to download for each supported desktop platform.
- Explain basic install or run steps for supported platforms.
- Explain checksum or signature verification when release assets provide it.
- Separate end-user installation from contributor development setup.
- Include a short safety note that the app manages e-reader files and users should follow backup and confirmation prompts.

## Non-Goals

- Full release workflow implementation in the initial local development change.
- Local contributor signing setup.
- Mobile release packaging.
- Store deployment.
- Platform-specific notarization unless added by a later release-signing change.
