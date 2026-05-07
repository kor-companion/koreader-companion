---
schema_version: 1
id: backup-restore
title: Backup And Restore
originating_changes:
  - CHG-2026-007-5870-implement-koreader-backup-manifest-engine
  - CHG-2026-008-b629-implement-selective-restore-and-backup-verification
revised_by_changes: []
---

# Backup And Restore

Backup and restore are first-class product capabilities, not secondary utilities. They create retention value after installation and build trust before risky operations.

## Backup Content

Backups should include supported KOReader data such as:

- KOReader settings files.
- `.sdr` directories.
- `metadata.lua` files.
- Reading positions.
- Highlights and annotations when present in backed-up metadata.
- Book-specific metadata.
- Installation-state files needed for rollback guidance when safe to copy.

## Manifest

SQLite should track:

- Backup set identity.
- Device identity and observed metadata.
- Source paths and relative paths.
- File sizes.
- Hashes.
- Modified timestamps.
- Backup timestamps.
- Operation log links.

## Restore

Restore must be selective and previewed.

- Show the source backup and destination device.
- Verify backup manifest integrity before restore.
- Validate destination path containment.
- Preview every write and overwrite.
- Offer backup-before-restore for affected destination files.
- Write logs for every restored file.
- Report skipped, changed, and failed files.
