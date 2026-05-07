---
schema_version: 1
id: capability-architecture
title: Capability Architecture
originating_changes:
  - CHG-2026-002-966f-build-headless-capability-foundation
revised_by_changes: []
---

# Capability Architecture

The app must be organized around host capabilities, device target capabilities, and reusable workflows. Kobo desktop is the first implementation, not a special case baked into the core. The foundational capability architecture must be usable without a production frontend.

## Required Interfaces

- Host access interface for mount discovery, manual path selection, filesystem reads and writes, sync, and eject.
- Device target interface for identification, support level, capability flags, install paths, backup paths, and readiness checks.
- Workflow interface for dry-run, execution, progress, logging, cancellation, and verification.
- Payload interface for release metadata, artifact selection, validation, staging, and extraction.
- Persistence interface for devices, backup manifests, hashes, timestamps, and operation logs.
- Safety interface for path containment, backup-before-write, confirmations, rollback guidance, and error reporting.
- Frontend-facing interface for consuming workflow/domain state after a frontend framework is selected.

## Capability Flags

Device and host implementations should expose explicit capabilities rather than relying on type checks.

- `canInstallKOReader`
- `canBackupKOReaderData`
- `canRestoreKOReaderData`
- `canPatchLauncherConfig`
- `requiresJailbreak`
- `requiresDeveloperMode`
- `supportsSafeEject`
- `supportsDirectFilesystemAccess`
- `supportsRemoteShell`
- `supportsAdbInstall`
- `supportsSelectiveRestore`

## Acceptance Criteria

- Adding a new target device does not require changing UI screens except to expose new capability text or target-specific guidance.
- Adding a new host platform does not require rewriting install, backup, or restore workflow logic.
- Workflows can run in dry-run mode without performing writes.
- Every write operation is attributable to a plan item and an operation log entry.
- The headless foundation can be validated without adding Flutter, Qt, Tauri, Electron, or another production frontend dependency.
