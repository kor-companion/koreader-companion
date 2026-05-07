---
schema_version: 1
id: defer-frontend-framework-selection
title: Defer Frontend Framework Selection
originating_changes:
  - CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation
related_changes:
  - CHG-2026-002-966f-build-headless-capability-foundation
---

# Defer Frontend Framework Selection

## Decision

Do not lock in Flutter, Qt, Tauri, Electron, native UI, or any other production frontend framework before the headless foundation and riskiest Kobo desktop workflows are implemented and validated.

## Rationale

The project risk is dominated by host/device integration and safety-critical filesystem behavior, not by rendering. Mount discovery, permissions, payload validation, path containment, backup-before-write, safe eject, ADB, SSH, and future Android host access all require platform-specific evidence.

The maintainer has Flutter and Material 3 experience, so Flutter remains a strong candidate. However, similar projects show that device-management tools usually rely on Qt/native stacks or high-level UI shells over native/system cores. Framework selection should therefore follow validated constraints rather than precede them.

## Consequences

- Early MVP implementation should be headless or diagnostic-friendly.
- Nix and release foundations should avoid Flutter/Dart assumptions until a frontend is selected.
- The core must expose domain workflow state and command boundaries that any frontend can consume.
- `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation` must capture the final frontend decision before production UI work begins.
