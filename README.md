# KOReader Companion

[![Status: alpha in progress](https://img.shields.io/badge/status-alpha%20in%20progress-orange)](runecontext/project/roadmap.md)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

KOReader Companion (`KORCompanion` in the project docs) is a planned desktop companion for people who want KOReader on supported e-readers without relying on fragile manual steps. The project aims to make KOReader installation, backup, restore, and device management safer, more repeatable, and easier to trust.

## Overview

The project is being designed as a capability-based companion platform:

- A shared Rust headless core owns discovery, workflow planning, payload validation, safety checks, backup and restore logic, logs, and persistence.
- Host platforms such as Linux, macOS, and Windows are added through host modules.
- Supported devices such as Kobo, and later other e-readers, are added through device modules.
- The architecture is intended to let new hosts and devices be added without rearchitecting the core workflows.

The MVP is intentionally narrow: Kobo target devices managed from desktop hosts.

## Who It Is For

The first users are not every e-reader owner.

- Kobo owners who want KOReader but are nervous about manual install guides
- Existing KOReader users who want safer backup and restore workflows
- Community helpers who want a transparent tool they can recommend with confidence

This repository is not yet a general-purpose e-reader suite, mobile app, or broad multi-device manager.

## MVP Direction

Publicly, the MVP should be understood as a **Kobo desktop companion for KOReader installation and backup**.

The first release is planned to focus on:

- Kobo device discovery and dry-run preflight
- KOReader release fetching and payload validation
- Safe Kobo installation with backup-before-write behavior
- Operation logs, rollback guidance, and safe eject where supported
- KOReader backup manifests and selective restore
- A user-facing desktop frontend shell built on top of the validated headless core

## Trust Boundaries And Non-Goals

The project only becomes useful if users and community reviewers can trust its behavior.

- No jailbreak or exploit automation
- No bundled proprietary firmware, vendor binaries, or vendor-owned assets
- No writes without an explicit user confirmation step
- No installation without a dry-run preflight plan
- No hidden "just trust us" automation for risky device operations
- No claim that all e-readers or all KOReader workflows are supported in the MVP

## Roadmap Direction

The current roadmap is intentionally sequenced from risky core work toward user-facing packaging:

1. Set up the Nix-based development environment.
2. Lock in the product foundation and trust boundaries.
3. Build the Rust headless capability-based core.
4. Implement Kobo discovery, release handling, installation, safety, backup, and restore workflows.
5. Evaluate the frontend approach after the headless workflows are proven.
6. Build the MVP frontend shell over the headless core.
7. Prepare a community beta and finalize release packaging/documentation.

After the Kobo desktop MVP, the roadmap expands toward additional targets such as PocketBook, already-compatible Kindle states, Android/ADB devices, reMarkable SSH workflows, Android mobile host support, and eventually broader multi-device management.

## Status

The repository is currently in the planning and architecture stage. The detailed project context, specs, decisions, and change roadmap live under `runecontext/`.

The first implemented foundation is the Nix flake development environment plus the initial repository verification surface. Contributor setup is available now; end-user installation guidance and signed release artifacts are planned for a later release change.

## Development Environment

- Contributor setup: `docs/development.md`
- Contribution policy and DCO requirements: `CONTRIBUTING.md`
- Enter the canonical dev shell: `nix develop`
- Run the fast repository check from inside the dev shell: `just ci-fast`
- Inspect flake outputs: `nix flake show`
- Validate the current repo baseline directly: `nix flake check`

Frontend-specific toolchains are intentionally deferred until the headless foundation and risky host/device workflows are validated.

## Contributing

See `CONTRIBUTING.md`. DCO sign-off is required for commits (`git commit -s`).

## Code Of Conduct

See `CODE_OF_CONDUCT.md`.

## Security

Please do not report vulnerabilities in public issues. See `SECURITY.md`.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
