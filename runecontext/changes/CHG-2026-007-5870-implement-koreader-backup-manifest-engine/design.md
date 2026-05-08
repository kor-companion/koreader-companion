# Design

## Overview
Implement a backup engine that copies supported KOReader and integration state into controlled backup directories, records each backup in SQLite, hashes files, and writes a human-readable summary beside the copied files.

## Default Backup Scope

Default MVP backups should include:

- KOReader settings and user data under `.adds/koreader` where they are user-owned or not trivially reinstallable.
- `.sdr` directories and `metadata.lua` files associated with reading progress, highlights, annotations, and book-specific metadata.
- User-installed KOReader assets such as dictionaries, fonts, plugins data, or comparable data when included by the selected backup profile.
- Kobo integration files modified by KORCompanion, including launcher config files and `.kobo/Kobo/Kobo eReader.conf` backups.
- Install-state files needed for rollback guidance when safe to copy.

Default MVP backups should not include books or a full copy of every reinstallable KOReader application file unless a later explicit backup profile adds that behavior.

## Manifest Shape

SQLite should track at least:

- Backup set identity, timestamp, profile, app/core version, and schema version.
- Device identity and observed metadata from CHG-003.
- Source root and source relative path.
- Backup relative path.
- File category, size, modified timestamp, hash, and copy status.
- Operation log link and rollback relationship when applicable.

Schema versioning and migration rules should be present from the first implementation.

## Human-Readable Summary

Each backup directory should contain a summary file, such as `backup-summary.json` or `backup-summary.yaml`, with:

- Backup set identity and timestamp.
- Device label or identifier.
- Backup profile/scope.
- File counts and total size.
- Source root summary.
- App/core version and manifest schema version.
- Notes about skipped files or warnings.

The summary is for user inspection. The SQLite manifest and file hashes remain the authoritative restore source.

## Safety Behavior

- All source and destination paths must be path-contained.
- Backup should tolerate unreadable optional files by recording skipped entries when safe, while failing closed for required rollback files.
- Hashes should be recorded after copy and verified against copied bytes.
- Backup operations should be cancellable without corrupting already completed backup sets.
