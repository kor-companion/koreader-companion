---
schema_version: 1
id: capability-architecture
title: Capability Architecture
originating_changes:
  - CHG-2026-002-966f-build-headless-capability-foundation
revised_by_changes:
  - CHG-2026-006-8e9f-implement-operation-safety-logs-rollback-guidance-and-ejection
  - CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core
---

# Capability Architecture

The app must be organized around host capabilities, device target capabilities, and reusable workflows. Kobo desktop is the first implementation, not a special case baked into the core. The foundational capability architecture must be usable without a production frontend.

The headless core should be implemented in Rust unless implementation research uncovers a strong reason to change course. Rust modules should expose capability-oriented interfaces for host access, device targets, payloads, workflows, persistence, and safety. Concrete host and device support should live behind those interfaces so future platforms can be added as modules.

## Required Interfaces

- Host access interface for mount discovery, manual path selection, filesystem reads and writes, sync, and eject.
- Device target interface for identification, support level, capability flags, install paths, backup paths, and readiness checks.
- Workflow interface for dry-run, execution, progress, logging, cancellation, and verification.
- Payload interface for release metadata, artifact selection, validation, staging, and extraction.
- Persistence interface for devices, backup manifests, hashes, timestamps, and operation logs.
- Safety interface for path containment, backup-before-write, confirmations, rollback guidance, and error reporting.
- Frontend-facing interface for consuming workflow/domain state after a frontend framework is selected.

## Module Expectations

- Shared workflow logic must operate on interfaces and capability flags, not hard-coded Kobo, Linux, macOS, Windows, or future platform branches.
- Host modules should own host-specific discovery, file access, permission behavior, sync, and eject semantics.
- Device modules should own target-specific sentinels, install paths, backup paths, launcher integration, readiness checks, and support limitations.
- Payload modules should own release source access, artifact matching, checksum validation, staging, and extraction.
- Persistence modules should own schema versioning, migrations, manifests, release metadata cache, device records, and logs.
- Safety modules should be reusable by install, backup, restore, and future configuration workflows.

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
- A new host platform or device target can be introduced by implementing the required Rust interfaces and fixtures without rewriting shared workflows.
