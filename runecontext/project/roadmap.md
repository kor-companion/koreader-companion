# Roadmap

This roadmap orders RuneContext changes by recommended implementation sequence. The MVP is Kobo target devices managed from desktop hosts. Post-MVP work expands toward the full KOReader companion vision.

The MVP should front-load risky, non-frontend foundations before selecting or implementing a production frontend. Frontend framework choice should be evidence-driven after host/device access, safety workflows, backup/restore, and release-payload behavior are validated.

## MVP

3. `CHG-2026-002-966f-build-headless-capability-foundation` - Build capability-based headless foundation.
   Create the non-frontend core, SQLite persistence, release service, device/host abstraction boundaries, workflow state model, and shared safety layer so later devices, mobile hosts, and frontend frameworks can be added without re-architecting the core.

4. `CHG-2026-003-776b-implement-kobo-desktop-discovery-and-dry-run-preflight` - Implement Kobo desktop discovery and dry-run preflight.
   Detect mounted Kobo devices on Linux, macOS, and Windows; support manual path selection where necessary; classify readiness; and present a no-write install or backup plan.

5. `CHG-2026-004-5d52-implement-koreader-release-fetching-and-payload-validation` - Implement KOReader release fetching and payload validation.
   Fetch release metadata, choose the correct Kobo artifact, validate downloads, stage extraction, and make payload decisions visible in the dry-run plan.

6. `CHG-2026-005-49bc-implement-safe-kobo-koreader-installation-workflow` - Implement safe Kobo KOReader installation workflow.
   Install KOReader and required Kobo launch integration through a deterministic workflow that backs up before writes, patches Kobo configuration safely, and fails closed on ambiguous device state.

7. `CHG-2026-006-8e9f-implement-operation-safety-logs-rollback-guidance-and-ejection` - Implement operation safety logs, rollback guidance, and ejection.
   Add action logs, confirmation gates, rollback instructions, OS sync or safe-eject behavior, and user-visible recovery information.

8. `CHG-2026-007-5870-implement-koreader-backup-manifest-engine` - Implement KOReader backup manifest engine.
   Back up KOReader settings, `.sdr` folders, `metadata.lua`, reading progress, highlights, and book-specific data using SQLite manifests with hashes and modified timestamps.

9. `CHG-2026-008-b629-implement-selective-restore-and-backup-verification` - Implement selective restore and backup verification.
   Let users inspect backups, verify manifests, preview restore operations, and restore selected KOReader data with path containment and backup-before-restore protections.

10. `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation` - Evaluate frontend framework after foundation validation.
     Use the validated headless core and risky workflow evidence to compare Flutter, Qt, Tauri, Electron, native, and hybrid approaches. Decide the frontend strategy and produce any follow-up frontend implementation change needed before beta packaging.

11. `CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core` - Build MVP frontend shell over headless core.
    Build the production MVP app surface after frontend evaluation selects an approach. Keep the UI as a shell over the Rust headless core: it consumes workflow state, displays plans/logs/progress, and sends commands through the selected integration boundary without owning filesystem, install, backup, restore, or eject logic.

12. `CHG-2026-009-e6bd-prepare-mvp-community-beta-and-release-packaging` - Prepare MVP community beta and release packaging.
    Package desktop builds, publish supported device and firmware notes, prepare onboarding and recovery documentation, and validate the MVP with community beta users.

13. `CHG-2026-018-d4f1-finalize-github-release-workflow-and-install-documentation` - Finalize GitHub release workflow and install documentation.
    Finish the MVP release path by adding a GitHub release workflow that consumes unsigned Nix artifacts, signs and publishes release assets, and documents end-user installation in `README.md`.

## Post-MVP

1. `CHG-2026-010-8a3b-add-narrow-koreader-configuration-management` - Add narrow KOReader configuration management.
   Add safe UI toggles for known KOReader settings using bounded, tested transformations. Do not implement a general Lua parser until the need and safety approach are proven.

2. `CHG-2026-011-f1f3-add-pocketbook-and-expanded-usb-mass-storage-targets` - Add PocketBook and expanded USB mass storage targets.
   Extend the USB mass storage target model beyond Kobo, prioritizing devices with direct filesystem workflows and lower legal or unlock complexity.

3. `CHG-2026-012-7f08-add-kindle-unlocked-state-detection-and-supported-install-flows` - Add Kindle unlocked-state detection and supported install flows.
   Support Kindle only for already-compatible unlocked states with concrete detection rules, no jailbreak automation, and explicit ToS, warranty, and data-risk warnings.

4. `CHG-2026-013-7337-add-android-adb-device-workflow` - Add Android ADB device workflow.
   Support Onyx Boox and Android-tablet KOReader APK workflows through an ADB driver that handles authorization, device state, APK install, and recovery guidance.

5. `CHG-2026-014-bd6b-add-remarkable-ssh-workflow-research-and-implementation` - Add reMarkable SSH workflow research and implementation.
   Research current firmware and package-manager constraints before implementing SSH-based workflows that remain supportable.

6. `CHG-2026-015-a715-research-and-add-android-mobile-host-support` - Research and add Android mobile host support.
   Validate USB OTG and Android Storage Access Framework behavior with real devices before implementing mobile-host workflows.

7. `CHG-2026-016-7bd5-add-cross-device-sync-and-advanced-management-suite` - Add cross-device sync and advanced management suite.
   Evolve the product into a multi-device KOReader management suite with migration, restore profiles, cross-device backup workflows, health checks, and advanced operations.

## Removed Superseded Work

The earlier empty Flutter-first foundation change directory, `CHG-2026-002-966f-build-capability-based-flutter-desktop-foundation`, was removed after being superseded by the headless core and frontend evaluation changes. The roadmap now tracks the active headless-first path only.

# Completed Changes
1. `CHG-2026-017-50a5-set-up-nix-local-development-and-release-artifacts` - Set up Nix local development and release artifacts.
   Add the Nix flake dev shell first so all MVP implementation work shares a pinned local environment. Use a root-level `nix/` folder for reusable Nix modules, allow nixpkgs unstable where practical, and shape unsigned release artifact outputs so a later GitHub release workflow can sign and publish them without reorganizing the Nix files. Do not assume Flutter, Dart, or any production frontend framework yet.

2. `CHG-2026-001-4104-define-korcompanion-product-foundation` - Define KORCompanion product foundation.
   Establish the vision, target users, product promise, non-goals, adoption strategy, legal/trust boundaries, and durable product language before implementation begins.


