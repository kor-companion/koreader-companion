# Design

## Overview

Evaluate frontend frameworks only after the headless foundation has validated the highest-risk product constraints.

The evaluation should answer whether the frontend should be Flutter, Qt, Tauri, Electron, native, or a hybrid shell over a native core. It should use project-specific evidence from implemented host access, workflow state, persistence, backup/restore, and safety behavior.

## Current Research Notes

Comparable open-source projects suggest that device-management tools usually succeed with either a native desktop stack or a high-level UI shell over native/system code.

| Project | Observed Stack | Relevance |
| --- | --- | --- |
| calibre | Python plus Qt with native components | Closest e-reader ecosystem analog; mature desktop-first e-book and device management. |
| qFlipper | C++ plus Qt/QML and USB/device libraries | Very similar desktop companion shape for hardware update and device workflows. |
| KDE Connect desktop | C++ plus Qt/QML and platform integration | Cross-device utility with deep OS integration and plugin-style architecture. |
| OpenRGB | C++ plus Qt with USB/HID/system access | Hardware-management app where native access and reliability dominate UI framework choice. |
| balenaEtcher | Electron/TypeScript plus native modules/sidecars | Shows a high-level UI can work when device-critical behavior is delegated to native/system components. |
| LocalSend | Flutter/Dart with platform-specific and native/Rust pieces | Strong evidence that Flutter can work well for cross-platform utility UX when native integrations are isolated. |
| OpenMTP | Electron/React plus native transfer layer | Reinforces the shell-plus-native-core pattern for file/device transfer apps. |
| Syncthing | Go core daemon plus web UI | Demonstrates reliability-first file synchronization with UI separated from core behavior. |

## Research Conclusions

- Flutter is viable for the UI, especially given existing Flutter and Material 3 experience.
- Flutter should not be treated as proof that the whole product can be Dart-first.
- The core product risk is host/device integration, not rendering performance.
- Similar device-management apps skew toward Qt/native or shell-over-native-core architectures.
- Future Android UI reuse is valuable but should not dominate the MVP decision because Android host support remains post-MVP research.

## Evaluation Criteria

- Developer experience for the actual maintainer and likely contributors.
- Ability to consume existing headless workflow state without moving safety logic into the UI.
- Native integration burden for mount discovery, filesystem permissions, safe eject, ADB, SSH, and future Android host access.
- Packaging, signing, notarization, and Linux distribution fit.
- End-user trust, native feel, accessibility, and transparency for a tool that writes to e-reader storage.
- Runtime performance for long operations, progress reporting, logs, backup browsing, and restore previews.
- Binary size and startup behavior for desktop utility users.
- Long-term maintainability of custom plugins, sidecars, or FFI boundaries.

## Recommendation To Revisit Later

The current recommendation is conditional:

- Use Flutter only if it is a frontend shell over a validated host/device core.
- Avoid a mostly pure-Flutter/Dart implementation for safety-critical host and device behavior.
- Prefer Qt/QML or a native-core architecture if validated host access becomes the dominant maintenance burden.

The final decision should be made in this change after the headless core and risky workflows exist.

## Follow-Up Implementation

The expected follow-up implementation change is `CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core`. CHG-019 should either confirm that change is ready to implement with the selected framework or replace it with a more accurate implementation split before CHG-009 packaging begins.
