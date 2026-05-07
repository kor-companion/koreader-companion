## Summary
Build capability-based headless foundation

## Problem
Create the non-frontend application foundation around host, device, workflow, persistence, release, and safety abstractions so Kobo is first without hard-coding Kobo-only assumptions.

The riskiest MVP work is host/device integration: mount discovery, filesystem permissions, path containment, payload validation, backup-before-write, OS sync or safe eject, and operation logging. Those capabilities should be implemented and validated before selecting or building a frontend framework.

## Proposed Change
Build a headless core that can produce dry-run plans, execute guarded workflows, persist manifests/logs, and expose workflow state through stable interfaces that any future frontend can consume.

Do not add Flutter, Material UI, Qt, Electron, Tauri, or other frontend framework commitments in this change.

## Why Now
The risky foundational behavior should be proven before frontend implementation begins. This keeps framework selection evidence-driven and prevents frontend assumptions from shaping the safety-critical core.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Initial validation can be performed through tests, fixtures, command-line entry points, or small diagnostic tools without a production frontend.
- Frontend framework selection is deferred to `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation`.

## Out of Scope
- Production desktop UI.
- Mobile UI.
- Frontend framework selection.
- Release-store packaging or app signing.

## Impact
This change makes the MVP less dependent on early UI-framework decisions and gives later frontend evaluation real implementation evidence.
