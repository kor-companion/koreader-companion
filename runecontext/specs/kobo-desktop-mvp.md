---
schema_version: 1
id: kobo-desktop-mvp
title: Kobo Desktop MVP
originating_changes:
  - CHG-2026-003-776b-implement-kobo-desktop-discovery-and-dry-run-preflight
  - CHG-2026-004-5d52-implement-koreader-release-fetching-and-payload-validation
  - CHG-2026-005-49bc-implement-safe-kobo-koreader-installation-workflow
revised_by_changes:
  - CHG-2026-006-8e9f-implement-operation-safety-logs-rollback-guidance-and-ejection
---

# Kobo Desktop MVP

The MVP must deliver a complete Kobo desktop workflow: detect, preflight, fetch, validate, install, log, and safely eject.

## Detection

- Detect Kobo-like mounted volumes by host metadata and sentinel files.
- Prefer the `KOBOeReader` filesystem label when the host exposes it, then verify sentinels before trusting the root.
- Require `.kobo/` and `.kobo/Kobo/Kobo eReader.conf` for install-capable classification. Treat `.kobo/` without the expected config path as ambiguous unless a later compatibility rule proves otherwise.
- Support Linux, macOS, and Windows through host-specific discovery adapters.
- Support manual path selection when automatic detection is ambiguous.
- Fail closed if the selected path does not look like a supported device.

## Preflight

- Identify detected device, support level, and known limitations.
- Show what will be read and written.
- Detect existing KOReader and launch integration state.
- Detect `.adds/koreader`, `.adds/koreader/koreader.sh`, `.adds/kfmon`, `.adds/nm`, launcher icons, and pending `.kobo/KoboRoot.tgz` when present.
- Warn when firmware, filesystem, free space, or permissions are risky or unknown.
- Produce a no-write dry-run plan before installation.

Dry-run output must identify every planned read and write, including `.adds/koreader`, launcher integration files, `.kobo/Kobo/Kobo eReader.conf`, backups, and logs.

## Installation

- Fetch and validate the Kobo KOReader artifact.
- Match KOReader Kobo release assets using the `koreader-kobo-v<version>.zip` naming pattern unless upstream changes require an updated matcher.
- Prefer GitHub release asset digests when available. If digests are unavailable, use published checksum metadata only when it can be parsed deterministically.
- Stage extraction before writing to the device.
- Validate the staged payload before writes. At minimum, require a top-level `koreader/` directory and executable launch script path expected by the selected launcher.
- Install KOReader into `.adds/koreader` on the Kobo storage root.
- Install or validate launch integration through a documented launcher path. KFMon and NickelMenu are the initial researched options; the implementation change must choose one default and document when an existing supported launcher can be reused.
- Patch required Kobo configuration safely without breaking existing settings. For firmware behavior that needs Nickel indexing exclusions, patch `.kobo/Kobo/Kobo eReader.conf` under `[FeatureSettings]` with `ExcludeSyncFolders=(\\.(?!kobo|adobe).+|([^.][^/]*/)+\\..+)` idempotently.
- Back up affected files before modification.
- Log every write.

Official Kobo firmware updates may affect launcher integration. The install workflow should detect missing or disabled launcher state after a firmware update and guide reinstallation without deleting KOReader user data.

## Completion

- Verify expected files exist after writes.
- Show success, warnings, and next steps.
- Run sync or safe-eject where supported.
- Tell the user when safe eject could not be confirmed.

Failure handling must be fail-closed. Abort before writes when sentinels are missing, release artifacts cannot be validated, configuration patches cannot be persisted, launcher payloads are ambiguous, or the staged payload does not match expectations. If partial writes may have occurred, show the affected paths and rollback guidance from the operation log.
