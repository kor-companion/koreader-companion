## Summary
Build MVP frontend shell over headless core

## Problem
CHG-019 selects a frontend approach, but the roadmap also needs an explicit implementation change between frontend evaluation and beta packaging. The MVP needs a user-facing shell that exposes the validated headless workflows without moving safety-critical logic into the UI.

## Proposed Change
Build the production MVP frontend shell on top of the Rust headless core after CHG-019 selects the frontend approach. This change is framework-neutral: it defines what the frontend must do regardless of whether the selected implementation is Flutter, Qt, Tauri, Electron, native, or another shell.

The frontend should consume domain workflow state, dry-run plans, progress events, operation logs, backup manifests, and rollback guidance from the core. It must not directly own device discovery, path containment, release selection, filesystem writes, backup/restore logic, or safe-eject behavior.

## Why Now
Packaging a community beta requires more than a framework recommendation. This change makes the actual MVP app surface explicit and prevents CHG-009 from assuming an untracked frontend implementation exists.

## Assumptions
- CHG-019 has selected a frontend approach and recorded its rationale.
- CHG-002 through CHG-008 provide validated headless workflows and domain state.
- The frontend remains a shell over the headless core, not a second implementation of core workflows.

## Out of Scope
- Reopening frontend framework selection.
- Moving safety-critical filesystem behavior into UI code.
- Adding post-MVP device families or mobile-host workflows.
- Store distribution or release signing.

## Impact
This change closes the roadmap gap between frontend evaluation and MVP beta packaging while preserving the headless architecture.
