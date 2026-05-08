---
schema_version: 1
id: kobo-desktop-first
title: Build Kobo Desktop First
originating_changes:
  - CHG-2026-001-4104-define-korcompanion-product-foundation
related_changes:
  - CHG-2026-002-966f-build-headless-capability-foundation
  - CHG-2026-003-776b-implement-kobo-desktop-discovery-and-dry-run-preflight
  - CHG-2026-005-49bc-implement-safe-kobo-koreader-installation-workflow
  - CHG-2026-006-8e9f-implement-operation-safety-logs-rollback-guidance-and-ejection
---

# Build Kobo Desktop First

## Decision

The first production-grade release should support Kobo target devices from desktop hosts before expanding to other devices or mobile host platforms.

## Rationale

Kobo provides the best initial balance of user value, implementation clarity, community relevance, and legal/trust simplicity. A narrow first release increases credibility while still supporting the broader companion-platform vision.

## Consequences

- MVP support claims must say Kobo desktop clearly.
- Architecture must remain capability-based so this choice does not become a permanent limitation.
- Other device families remain planned but blocked on explicit research and support criteria.
- Production frontend selection remains deferred until the Kobo desktop risks are validated through the headless foundation.
