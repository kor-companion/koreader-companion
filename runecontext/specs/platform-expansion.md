---
schema_version: 1
id: platform-expansion
title: Platform Expansion
originating_changes:
  - CHG-2026-011-f1f3-add-pocketbook-and-expanded-usb-mass-storage-targets
  - CHG-2026-012-7f08-add-kindle-unlocked-state-detection-and-supported-install-flows
  - CHG-2026-013-7337-add-android-adb-device-workflow
  - CHG-2026-014-bd6b-add-remarkable-ssh-workflow-research-and-implementation
  - CHG-2026-015-a715-research-and-add-android-mobile-host-support
revised_by_changes: []
---

# Platform Expansion

Post-MVP platform expansion should be evidence-driven. New platforms must be added only after their access model, safety model, and legal/trust boundaries are clear.

## Expansion Order

1. Additional USB mass storage devices with direct filesystem workflows.
2. Kindle only for already-compatible unlocked states with concrete detection rules.
3. ADB-managed Android-based devices.
4. reMarkable SSH workflows only after current firmware supportability is validated.
5. Android mobile host support only after USB OTG and Storage Access Framework research.

## Entry Criteria

- A documented device detection strategy.
- A documented install or backup path.
- A dry-run plan that can be produced without writes.
- A safety model for backup-before-write and rollback guidance.
- A legal/trust assessment for unlock, developer mode, warranty, and vendor ToS concerns.
- Test coverage with at least one real representative device before public support claims.
