# Design

## Overview
This change is intentionally not implementation-ready. Cross-device sync and advanced management should remain a roadmap placeholder until single-device backup/restore, restore verification, and at least one additional device target are proven.

## Required Future Split

Before implementation, split this into focused changes such as:

- Device migration between compatible targets.
- Restore profiles and backup profiles.
- KOReader metadata conflict research.
- Conflict-resolution policy for reading progress, highlights, annotations, and settings.
- Multi-device health checks.
- Optional sync transport if local USB workflows are not sufficient.

## Research Questions

- Which KOReader metadata files are safe to merge and which must be restored as whole files?
- How should conflicts between reading progress, highlights, bookmarks, and settings be represented to users?
- Which devices expose enough stable identifiers to support safe migration?
- What can be shared across device targets without violating platform-specific safety constraints?
