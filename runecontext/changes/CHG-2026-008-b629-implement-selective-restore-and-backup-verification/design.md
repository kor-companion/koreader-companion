# Design

## Overview
Implement restore as a guarded workflow over verified backup manifests. Users should be able to inspect a backup, select supported restore categories, preview writes, and restore only after destination checks and confirmation.

## Backup Verification

- Load the backup set from SQLite and cross-check the human-readable summary when present.
- Verify manifest schema compatibility.
- Verify file hashes for selected restore inputs before planning writes.
- Report missing, changed, skipped, and unverifiable files.
- Block restore when required manifest or file integrity checks fail.

## Restore Planning

The restore plan should show:

- Source backup set, timestamp, profile, and device metadata.
- Destination device root and support classification.
- Selected categories and files.
- Every destination write and overwrite.
- Destination files that will be backed up before restore.
- Files that are skipped because they are out of scope, unsafe, missing, or incompatible.

## Destination Safety

- Validate destination path containment for every restore item.
- Require a compatible destination device target before writing.
- Offer backup-before-restore for every destination file that will be replaced.
- Treat KOReader Lua files as complete opaque files unless a later narrow configuration change allows safe transformations.
- Do not merge highlights, reading positions, or settings in MVP; restore selected files or skip with an explanation.

## Operation Logging

Restore should link to CHG-006 operation logs and record verification, confirmation, writes, backup-before-restore records, skipped files, failures, and rollback guidance.
