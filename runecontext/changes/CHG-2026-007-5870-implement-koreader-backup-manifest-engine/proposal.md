## Summary
Implement KOReader backup manifest engine

## Problem
Back up KOReader settings, SDR metadata, reading progress, highlights, and book-specific data into a SQLite-tracked manifest with hashes and timestamps.

## Proposed Change
Implement the Rust backup engine and SQLite manifest schema for Kobo MVP backups. The default backup profile should prioritize user-owned KOReader state and files modified by KORCompanion, not blindly snapshot every reinstallable application binary.

Each backup set should include a SQLite manifest record and a human-readable summary file so users can identify backup contents even if the app database is unavailable.

## Why Now
Backup-before-write and long-term trust require a reliable manifest engine before selective restore and before broad community testing.

## Assumptions
- CHG-002 provides persistence and safety boundaries.
- CHG-003 provides detected Kobo roots and backup path discovery.
- KOReader Lua files are treated as opaque backup artifacts for MVP.

## Out of Scope
- Selective restore execution.
- General Lua parsing or arbitrary KOReader setting edits.
- Cloud or cross-device sync.
- Backing up books by default.

## Impact
This change creates the data integrity foundation for restore, rollback, migration, and future multi-device management.
