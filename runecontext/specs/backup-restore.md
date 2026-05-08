---
schema_version: 1
id: backup-restore
title: Backup And Restore
originating_changes:
  - CHG-2026-007-5870-implement-koreader-backup-manifest-engine
  - CHG-2026-008-b629-implement-selective-restore-and-backup-verification
revised_by_changes:
  - CHG-2026-010-8a3b-add-narrow-koreader-configuration-management
  - CHG-2026-016-7bd5-add-cross-device-sync-and-advanced-management-suite
---

# Backup And Restore

Backup and restore are first-class product capabilities, not secondary utilities. They create retention value after installation and build trust before risky operations.

## Backup Content

MVP backups should prioritize user-owned state and install state needed for safe rollback. They should not blindly snapshot every application binary unless a specific workflow requests a full install snapshot.

Default backup scope should include supported KOReader data such as:

- KOReader settings files.
- User-installed KOReader data that is practical to copy, such as dictionaries, fonts, plugins data, and other non-release user assets when included by the selected backup profile.
- `.sdr` directories.
- `metadata.lua` files.
- Reading positions.
- Highlights and annotations when present in backed-up metadata.
- Book-specific metadata.
- Installation-state files needed for rollback guidance when safe to copy.
- Kobo launcher and configuration files modified by KORCompanion, such as launcher config entries and `.kobo/Kobo/Kobo eReader.conf` backups.

Backup scope should distinguish these categories:

- User data: settings, reading metadata, annotations, dictionaries, fonts, and user-installed data.
- Device integration state: launcher configuration and Kobo configuration files touched by KORCompanion.
- Reinstallable application files: KOReader release binaries and bundled application files that can be restored by reinstalling the validated release artifact.
- Books: out of scope by default unless a future backup profile explicitly includes them.

The default MVP backup should favor user data and modified integration state. A full install snapshot can be added later if storage, duration, and restore semantics are made explicit.

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

Each backup directory should also include a human-readable summary file, such as JSON or YAML, containing the backup set identity, device label or identifier, timestamp, profile/scope, file counts, total size, source root, and app/version metadata. This file is not the source of truth when SQLite is available, but it lets users inspect backups if the app database is unavailable.

## Restore

Restore must be selective and previewed.

- Show the source backup and destination device.
- Verify backup manifest integrity before restore.
- Validate destination path containment.
- Preview every write and overwrite.
- Offer backup-before-restore for affected destination files.
- Write logs for every restored file.
- Report skipped, changed, and failed files.
- Use the human-readable summary to help users identify backup contents, while verifying actual restore integrity against the SQLite manifest and file hashes.
