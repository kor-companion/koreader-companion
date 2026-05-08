## Summary
Implement Kobo desktop discovery and dry-run preflight

## Problem
Detect mounted Kobo devices on Linux, macOS, and Windows, classify readiness, support manual path selection, and produce a no-write preflight plan before any install or backup workflow.

## Proposed Change
Implement the first Kobo device target module and desktop host discovery path over the Rust headless foundation.

The change should detect candidate Kobo USB mass-storage roots, verify sentinels, classify support level, detect existing KOReader and launcher state, and produce a no-write dry-run plan. Automatic discovery should prefer the `KOBOeReader` volume label when available, then verify `.kobo/` and `.kobo/Kobo/Kobo eReader.conf`. Manual path selection must run the same validation and fail closed on ambiguous roots.

## Why Now
This is the first real device/host validation path. It proves the capability architecture can support host-specific discovery while keeping shared workflow logic independent of Kobo-specific paths.

## Assumptions
- The headless core and capability boundaries from CHG-002 exist or are implemented with this change.
- Detection must be read-only.
- Kobo-specific paths remain inside the Kobo device target module.

## Out of Scope
- Downloading or validating release artifacts.
- Writing to the device.
- Installing KOReader or launchers.
- Backup and restore execution.

## Impact
The change gives later install, backup, and frontend work a trustworthy detected-device model and dry-run plan without allowing writes.
