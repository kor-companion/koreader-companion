---
schema_version: 1
id: product-scope
title: Product Scope
originating_changes:
  - CHG-2026-001-4104-define-korcompanion-product-foundation
revised_by_changes: []
---

# Product Scope

KORCompanion is a KOReader companion application. The long-term scope includes installation, backup, restore, configuration, and multi-device management. The MVP scope is Kobo target devices managed from desktop hosts.

## MVP Scope

- Desktop host app for Linux, macOS, and Windows.
- Kobo target detection over USB mass storage.
- Headless foundation and risky workflow validation before production frontend selection.
- Dry-run preflight for every install and restore operation.
- KOReader release fetching and Kobo artifact selection.
- Kobo launch integration setup.
- Safe Kobo configuration patching for required install safety.
- Backup before writes.
- Action logs and rollback guidance.
- Safe sync or eject behavior where the host supports it.
- KOReader data backup with manifests.
- Selective restore from verified backups.

## Post-MVP Scope

- Narrow KOReader settings management.
- Additional USB mass storage devices.
- Kindle support only for already-compatible unlocked states.
- ADB workflow for Onyx Boox and Android tablets.
- SSH workflow for reMarkable only after current supportability is validated.
- Android mobile host support only after USB OTG and Storage Access Framework research.
- Cross-device sync and advanced management.

## Frontend Selection

- Do not lock in a production frontend framework before the headless foundation and risky Kobo desktop workflows are validated.
- Evaluate Flutter, Qt, Tauri, Electron, native, and hybrid approaches after the core constraints are known.
- The selected frontend must consume workflow/domain state rather than own safety-critical filesystem or install behavior.

## Out Of Scope For MVP

- Jailbreak automation.
- Exploit distribution.
- Kindle installation.
- reMarkable installation.
- ADB installation.
- Android mobile host support.
- General Lua parser or arbitrary Lua config round-tripping.
- Cloud sync.
- Vendor firmware management.
- Production frontend implementation before foundational host/device risks are validated.
