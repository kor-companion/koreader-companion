---
schema_version: 1
id: defer-general-lua-parser
title: Defer General Lua Parser
originating_changes:
  - CHG-2026-010-8a3b-add-narrow-koreader-configuration-management
related_changes:
  - CHG-2026-007-5870-implement-koreader-backup-manifest-engine
  - CHG-2026-008-b629-implement-selective-restore-and-backup-verification
  - CHG-2026-016-7bd5-add-cross-device-sync-and-advanced-management-suite
---

# Defer General Lua Parser

## Decision

Do not implement a general Lua parser in the selected implementation stack for MVP. Treat KOReader Lua settings as opaque files for backup and restore. Post-MVP configuration management should begin with narrow, tested transformations for known settings.

## Rationale

General Lua parsing and safe round-tripping are high-risk because malformed config writes could break user setups. Backup and restore provide value without requiring arbitrary Lua mutation.

## Consequences

- MVP backup can include Lua files without interpreting them.
- MVP restore can restore complete files or selected backed-up data where safe.
- Config editing must be added slowly and covered by fixtures.
