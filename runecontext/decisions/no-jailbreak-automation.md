---
schema_version: 1
id: no-jailbreak-automation
title: Do Not Automate Jailbreaking
originating_changes:
  - CHG-2026-012-7f08-add-kindle-unlocked-state-detection-and-supported-install-flows
related_changes:
  - CHG-2026-001-4104-define-korcompanion-product-foundation
---

# Do Not Automate Jailbreaking

## Decision

KORCompanion must not bundle, distribute, or automate jailbreaks, exploits, locked-bootloader bypasses, or circumvention payloads.

## Rationale

The project should remain an interoperability and management utility. For locked devices, it can detect compatible states and link users to community documentation, but it should not perform or package the unlocking process.

## Consequences

- Kindle installation is post-MVP.
- Kindle support requires concrete unlocked-state detection rules.
- If compatible state cannot be verified, installation must halt.
- User warnings must distinguish warranty, ToS, data-loss, and compatibility risks.
