---
schema_version: 1
id: kobo-desktop-mvp
title: Kobo Desktop MVP
originating_changes:
  - CHG-2026-003-776b-implement-kobo-desktop-discovery-and-dry-run-preflight
  - CHG-2026-004-5d52-implement-koreader-release-fetching-and-payload-validation
  - CHG-2026-005-49bc-implement-safe-kobo-koreader-installation-workflow
revised_by_changes: []
---

# Kobo Desktop MVP

The MVP must deliver a complete Kobo desktop workflow: detect, preflight, fetch, validate, install, log, and safely eject.

## Detection

- Detect Kobo-like mounted volumes by sentinel files and directory structure.
- Support Linux, macOS, and Windows through host-specific discovery adapters.
- Support manual path selection when automatic detection is ambiguous.
- Fail closed if the selected path does not look like a supported device.

## Preflight

- Identify detected device, support level, and known limitations.
- Show what will be read and written.
- Detect existing KOReader and launch integration state.
- Warn when firmware, filesystem, free space, or permissions are risky or unknown.
- Produce a no-write dry-run plan before installation.

## Installation

- Fetch and validate the Kobo KOReader artifact.
- Stage extraction before writing to the device.
- Install KOReader into the expected device location.
- Install or validate launch integration.
- Patch required Kobo configuration safely without breaking existing settings.
- Back up affected files before modification.
- Log every write.

## Completion

- Verify expected files exist after writes.
- Show success, warnings, and next steps.
- Run sync or safe-eject where supported.
- Tell the user when safe eject could not be confirmed.
