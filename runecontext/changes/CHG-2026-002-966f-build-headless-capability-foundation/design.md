# Design

## Overview
Build the foundational core as headless, testable modules before committing to a frontend framework.

The core should model host capabilities, device targets, workflows, persistence, payload handling, and safety behavior through interfaces that a later frontend can consume without owning filesystem or install logic.

## Required Boundaries

- Host access owns mount discovery, manual path validation, filesystem permissions, OS sync, safe eject, and host-specific failure handling.
- Device targets own identification, support level, install paths, backup paths, and target capability flags.
- Workflows own dry-run planning, execution state, cancellation, progress, verification, and logs.
- Safety owns path containment, backup-before-write enforcement, confirmation requirements, rollback guidance, and fail-closed behavior.
- Persistence owns SQLite-backed manifests, known devices, hashes, timestamps, release metadata cache, acknowledgements, and operation logs.

## Frontend Deferral

No production frontend framework should be selected or embedded here. The core should expose plain domain state and command interfaces so a later Flutter, Qt, Tauri, Electron, native, or hybrid frontend can be evaluated against working backend constraints.

## Validation Approach

- Use fixtures and tests for path containment, plan generation, manifest behavior, and payload selection.
- Use representative host/device diagnostics for removable-storage discovery and safe-eject behavior.
- Keep all writes attributable to plan items and logs before any UI is built.
