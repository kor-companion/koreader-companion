## Summary
Implement operation safety logs rollback guidance and ejection

## Problem
Record action logs, enforce write confirmations, provide rollback guidance, and perform OS-level sync or safe-eject steps before device removal.

## Proposed Change
Integrate the shared safety layer into user-visible operation records, rollback guidance, confirmation gates, sync, and eject completion behavior. CHG-002 owns the safety primitives and CHG-005 uses them during install; this change makes the safety record complete and inspectable across install, backup, and restore workflows.

## Why Now
Device-writing workflows need auditable logs and recovery guidance before the project can ask users to trust it with e-reader storage.

## Assumptions
- CHG-002 provides shared safety and operation state primitives.
- CHG-005 provides the first write workflow that exercises logs, rollback guidance, and eject behavior.

## Out of Scope
- Implementing the Kobo install copy/patch workflow itself.
- Backup manifest design beyond log linkage.
- Production frontend framework selection.

## Impact
This change turns safety behavior into user-verifiable evidence: what was planned, what was written, what was backed up, what failed, and what the user should do next.
