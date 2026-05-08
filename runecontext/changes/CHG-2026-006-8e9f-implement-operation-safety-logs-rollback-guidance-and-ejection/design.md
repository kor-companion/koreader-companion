# Design

## Overview
Implement durable operation logs, rollback guidance rendering, confirmation gates, and host sync/eject completion behavior around the shared workflow and safety primitives.

## Scope Boundary

This change should not duplicate install logic from CHG-005. The boundary is:

- CHG-002 defines safety primitives and operation state.
- CHG-005 applies those primitives to the Kobo install workflow.
- CHG-006 makes the resulting logs, confirmations, rollback instructions, warnings, and eject outcomes complete and reusable.

## Operation Logs

Logs should record:

- Operation identity, workflow type, timestamps, app/core version, host adapter, and device target.
- Dry-run plan items and confirmation state.
- Every read and write that matters to user safety.
- Backup records for files modified or replaced.
- Verification steps and outcomes.
- Sync/eject attempts and outcomes.
- Errors, skipped steps, partial-write markers, and recovery guidance references.

## Confirmation Gates

- Confirm before any device write.
- Require context-specific acknowledgement for install, restore, configuration patching, and launcher changes.
- Never treat one acknowledgement as permanent approval for unrelated risk categories.

## Rollback Guidance

Rollback guidance should be generated from backup records and operation logs. It should list affected files, backup locations, whether automated restore is available, and manual recovery steps when automation is unsafe.

## Sync And Eject

- Use host adapters for OS sync and safe eject where available.
- Distinguish successful eject, sync-only completion, unsupported safe eject, user-cancelled eject, and failed eject.
- Warn clearly when safe eject cannot be confirmed.
- Keep enough diagnostics to improve host-specific adapters later.
