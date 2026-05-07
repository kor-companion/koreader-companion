---
schema_version: 1
id: capability-based-architecture
title: Use Capability-Based Architecture
originating_changes:
  - CHG-2026-002-966f-build-headless-capability-foundation
related_changes:
  - CHG-2026-011-f1f3-add-pocketbook-and-expanded-usb-mass-storage-targets
  - CHG-2026-012-7f08-add-kindle-unlocked-state-detection-and-supported-install-flows
  - CHG-2026-013-7337-add-android-adb-device-workflow
  - CHG-2026-014-bd6b-add-remarkable-ssh-workflow-research-and-implementation
  - CHG-2026-015-a715-research-and-add-android-mobile-host-support
---

# Use Capability-Based Architecture

## Decision

Represent host and device support through explicit capabilities and driver interfaces instead of brand-specific conditionals spread through the app.

## Rationale

The product vision includes many device families and host platforms. Capability contracts make Kobo-first implementation compatible with later expansion.

## Consequences

- New device support should arrive as a target implementation plus tests.
- New host support should arrive as a host access implementation plus tests.
- Core workflows should operate on capabilities, plans, and safety checks.
- Frontend implementations should consume capability and workflow state rather than own host/device safety behavior.
