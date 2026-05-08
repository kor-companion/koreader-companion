## Summary
Implement selective restore and backup verification

## Problem
Restore selected KOReader data from verified manifests, validate destination safety, and report exactly what will be changed before writes occur.

## Proposed Change
Implement selective restore over the backup manifest engine. The workflow should verify backup integrity, validate destination device compatibility, preview every write, offer backup-before-restore for affected destination files, restore selected files with path containment, and report skipped or changed files.

## Why Now
Restore is the proof that backups are useful. It must be safe, selective, previewed, and auditable before backup is presented as a trust feature.

## Assumptions
- CHG-007 provides backup manifests, hashes, and human-readable summaries.
- CHG-006 provides operation logs and rollback guidance.
- KOReader Lua files remain opaque restore artifacts unless a later narrow configuration change supports safe transformations.

## Out of Scope
- Editing or merging arbitrary Lua configuration.
- Cross-device conflict resolution or sync.
- Restoring books by default unless included by a future explicit profile.

## Impact
This change completes the MVP backup/restore loop and proves the manifest format can protect user data during writes.
