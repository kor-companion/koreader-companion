# Design

## Overview
Build the foundational core as headless, testable Rust modules before committing to a frontend framework.

The core should model host capabilities, device targets, workflows, persistence, payload handling, and safety behavior through interfaces that a later frontend can consume without owning filesystem or install logic.

Rust is selected as the starting implementation language because this project prioritizes safety-critical filesystem behavior, system integration, explicit errors, packaging, and frontend neutrality. This selection should be revisited only if Kobo implementation research uncovers a concrete blocker.

## Rust Workspace Direction

Exact crate names may change during implementation, but the workspace should preserve these boundaries:

- Core domain crate for capability contracts, workflow plans, state machines, domain errors, and user-facing events.
- Persistence crate or module for SQLite schemas, migrations, release metadata cache, device records, manifests, and operation logs.
- Payload crate or module for release metadata, artifact matching, checksum validation, staging, and extraction.
- Host adapter crates or modules for Linux, macOS, Windows, and future Android or iOS host behavior.
- Device target crates or modules for Kobo first, then later PocketBook, Kindle-compatible states, Android/ADB devices, reMarkable/SSH devices, and future targets.
- CLI or diagnostic crate for exercising discovery, dry-run, install, backup, restore, and eject flows before production frontend work.

## Required Boundaries

- Host access owns mount discovery, manual path validation, filesystem permissions, OS sync, safe eject, and host-specific failure handling.
- Device targets own identification, support level, install paths, backup paths, and target capability flags.
- Workflows own dry-run planning, execution state, cancellation, progress, verification, and logs.
- Safety owns path containment, backup-before-write enforcement, confirmation requirements, rollback guidance, and fail-closed behavior.
- Persistence owns SQLite-backed manifests, known devices, hashes, timestamps, release metadata cache, acknowledgements, and operation logs.

Shared workflows must depend on Rust traits or equivalent capability contracts, not concrete Kobo or desktop-host implementations. A new host or device should be added by implementing interfaces plus fixtures and integration checks.

## Frontend Deferral

No production frontend framework should be selected or embedded here. The core should expose plain domain state and command interfaces so a later Flutter, Qt, Tauri, Electron, native, or hybrid frontend can be evaluated against working backend constraints.

## Validation Approach

- Use fixtures and tests for path containment, plan generation, manifest behavior, and payload selection.
- Use representative host/device diagnostics for removable-storage discovery and safe-eject behavior.
- Keep all writes attributable to plan items and logs before any UI is built.
- Confirm the core builds and tests through standard Rust commands and through the Nix dev shell once CHG-017 is implemented.
