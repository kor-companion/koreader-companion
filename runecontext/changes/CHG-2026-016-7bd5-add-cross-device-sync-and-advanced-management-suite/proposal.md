## Summary
Add cross-device sync and advanced management suite

## Problem
Evolve KORCompanion into a multi-device management suite with cross-device backups, restore profiles, device migration, health checks, and advanced KOReader operations.

## Proposed Change
Keep this as a post-MVP vision placeholder until backup, restore, and at least one additional device target are proven. Before implementation, split this into smaller changes for migration, restore profiles, sync conflict policy, health checks, and any network or multi-device transport behavior.

## Why Now
The roadmap should preserve the long-term product direction, but this work is too broad to implement as a single change today.

## Assumptions
- CHG-007 and CHG-008 establish single-device backup and restore first.
- At least one post-MVP additional device target should exist before cross-device sync design begins.
- KOReader metadata conflict behavior requires dedicated research before implementation.

## Out of Scope
- Immediate implementation.
- Cloud sync commitments.
- Conflict-resolution policy before KOReader metadata research.
- Network transport or account systems.

## Impact
This keeps the ambition visible without implying that cross-device sync is implementation-ready.
