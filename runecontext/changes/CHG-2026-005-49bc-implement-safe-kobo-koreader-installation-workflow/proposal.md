## Summary
Implement safe Kobo KOReader installation workflow

## Problem
Install KOReader and required Kobo launch integration through a deterministic workflow that backs up before writing and patches required configuration safely.

## Proposed Change
Implement the guarded Kobo install workflow over the shared Rust workflow and safety layers. The workflow should consume a detected Kobo root from CHG-003 and a validated staged payload from CHG-004, then produce an explicit plan, require confirmation, back up affected files, copy KOReader into `.adds/koreader`, install or validate launcher integration, patch required Kobo configuration, verify writes, and log every action.

The change must choose the initial supported launcher path for new installs after validating current upstream guidance. KFMon and NickelMenu are the researched options. Existing supported launcher state may be reused when detected and safe.

## Why Now
This is the first safety-critical write workflow. It must prove backup-before-write, path containment, staged payload application, configuration patching, and fail-closed behavior before community beta work.

## Assumptions
- CHG-003 provides a supported Kobo device root and dry-run state.
- CHG-004 provides validated staged KOReader payloads.
- CHG-002 provides shared workflow, persistence, and safety primitives.

## Out of Scope
- Release metadata fetching and payload selection.
- Backup browsing and selective restore UI.
- General KOReader Lua configuration editing.
- Support for non-Kobo targets.

## Impact
The change establishes the trust model for all future device-writing workflows.
