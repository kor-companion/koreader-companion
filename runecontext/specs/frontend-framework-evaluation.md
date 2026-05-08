---
schema_version: 1
id: frontend-framework-evaluation
title: Frontend Framework Evaluation
originating_changes:
  - CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation
revised_by_changes:
  - CHG-2026-009-e6bd-prepare-mvp-community-beta-and-release-packaging
  - CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core
---

# Frontend Framework Evaluation

Frontend framework selection must be evidence-driven and must happen after the headless foundation validates the riskiest host/device workflows.

## Required Inputs

- Working host access boundaries for discovery, manual path validation, filesystem permissions, OS sync, and safe eject where supported.
- Working device target boundaries for Kobo identification, support classification, install paths, and backup paths.
- Working workflow state for dry-run, execution, progress, logs, cancellation, and verification.
- Working persistence for backup manifests, device records, hashes, timestamps, release metadata, and operation logs.
- Working safety behavior for path containment, backup-before-write, confirmations, rollback guidance, and fail-closed errors.

## Candidate Frameworks

The evaluation must consider:

- Flutter with Material 3.
- Qt/QML or Qt Widgets.
- Tauri with a native core.
- Electron with native modules or sidecars.
- Native-platform or hybrid approaches if the validated core suggests them.

## Decision Criteria

- The selected frontend must consume domain workflow state rather than own filesystem or install logic.
- Developer experience must account for maintainer experience, contributor likelihood, testing, debugging, and multi-language bridge complexity.
- End-user experience must prioritize trust, clear dry-run previews, accessibility, native platform expectations, reliable progress reporting, and recovery guidance.
- Performance assessment must include long backup scans, hashing, restore previews, log rendering, and large backup browsing without UI stalls.
- Packaging assessment must include Linux, macOS, and Windows release artifacts, signing, notarization, dependency bundling, and update/release documentation.
- Future Android reuse may be considered, but it must not override MVP desktop reliability.

## Output

The evaluation must produce:

- A written comparison of candidates.
- A recommended frontend approach.
- A rationale that explicitly states why Flutter was accepted or rejected.
- Confirmation that `CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core` remains the right frontend implementation change, or a replacement change if evaluation evidence shows a different implementation split is needed.
