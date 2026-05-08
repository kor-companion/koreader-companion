## Summary
Build capability-based headless foundation

## Problem
Create the non-frontend application foundation around host, device, workflow, persistence, release, and safety abstractions so Kobo is first without hard-coding Kobo-only assumptions.

The riskiest MVP work is host/device integration: mount discovery, filesystem permissions, path containment, payload validation, backup-before-write, OS sync or safe eject, and operation logging. Those capabilities should be implemented and validated before selecting or building a frontend framework.

## Proposed Change
Build a Rust headless core that can produce dry-run plans, execute guarded workflows, persist manifests/logs, and expose workflow state through stable interfaces that any future frontend can consume.

The core should be organized around shared workflow logic plus host and device modules. Linux, macOS, Windows, and later mobile hosts should implement host access interfaces. Kobo and later device families should implement device target interfaces. Install, backup, restore, release selection, logging, and safety behavior should remain shared.

Implementation now includes a modular Rust workspace with focused crates for domain contracts, payload handling, SQLite persistence, host access, device targets, and a diagnostic CLI. The core now uses address abstractions that can represent local filesystem paths, scoped transport-relative paths, remote endpoints, and logical identifiers so future ADB, SSH, and mobile-host work does not have to be forced through plain local paths.

This change also establishes an explicit repository source-size policy so the workspace stays split into logical modules and does not regress toward monolithic multi-thousand-line implementation files.

Do not add Flutter, Material UI, Qt, Electron, Tauri, or other frontend framework commitments in this change.

## Why Now
The risky foundational behavior should be proven before frontend implementation begins. This keeps framework selection evidence-driven and prevents frontend assumptions from shaping the safety-critical core.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Rust is the default headless-core language unless Kobo implementation evidence uncovers a strong reason to change course.
- Initial validation can be performed through tests, fixtures, command-line entry points, or small diagnostic tools without a production frontend.
- Frontend framework selection is deferred to `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation`.

## Out of Scope
- Production desktop UI.
- Mobile UI.
- Frontend framework selection.
- Release-store packaging or app signing.

## Impact
This change makes the MVP less dependent on early UI-framework decisions and gives later frontend evaluation real implementation evidence.

It also establishes the capability and module boundaries that later host/device-specific changes can extend without rewriting shared workflow, persistence, payload-selection, or safety logic.
