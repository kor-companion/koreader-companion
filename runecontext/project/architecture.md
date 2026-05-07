# Architecture

KORCompanion should be built as a capability-based companion platform. Kobo desktop support is the first implementation, but the core architecture must not hard-code assumptions that prevent later device targets or mobile host platforms.

## Core Principle

Model capabilities and workflows separately from device brands and host platforms.

The app should answer these questions through explicit interfaces:

- What host capabilities are available?
- What device was detected?
- What operations does this device support from this host?
- What unlock, developer-mode, filesystem, ADB, SSH, or permission state is required?
- What files will be read or written?
- What safety steps must run before and after writes?

## Layers

### Future Frontend Layer

Responsibilities:

- Onboarding and device connection guidance.
- Device status, readiness, and support explanations.
- Dry-run plans and write confirmations.
- Progress, logs, warnings, and recovery guidance.
- Backup browsing and selective restore UI.

The production frontend framework is intentionally deferred until the headless foundation and risky Kobo desktop workflows are validated. The eventual UI should consume domain workflow state rather than perform filesystem or install logic directly.

Candidate approaches include Flutter, Qt, Tauri, Electron, native UI, or a hybrid shell over a native core. Flutter remains a serious candidate because the maintainer has Flutter and Material 3 experience, but it is not selected by default.

### Domain Workflow Layer

Responsibilities:

- Install plans.
- Backup plans.
- Restore plans.
- Validation and preflight checks.
- Operation state machines.
- Rollback and recovery guidance.
- User-facing safety summaries.

This layer should coordinate drivers but avoid device-specific path constants except through target-specific adapters.

### Device Target Layer

Initial target:

- Kobo over USB mass storage.

Future targets:

- PocketBook over USB mass storage.
- Kindle over USB mass storage when already in a compatible unlocked state.
- Onyx Boox and Android tablets through ADB.
- reMarkable through SSH only where current firmware support is validated.

Each target implementation should expose capabilities such as:

- `canInstallKOReader`
- `canBackupKOReaderData`
- `canRestoreKOReaderData`
- `canPatchLauncherConfig`
- `requiresJailbreak`
- `requiresDeveloperMode`
- `supportsSafeEject`
- `supportsDirectFilesystemAccess`
- `supportsRemoteShell`
- `supportsAdbInstall`
- `supportsSelectiveRestore`

### Host Access Layer

Initial hosts:

- Linux desktop.
- macOS desktop.
- Windows desktop.

Future hosts:

- Android mobile host through USB OTG and Storage Access Framework only after a research spike proves hidden directories, traversal, write permissions, and performance are acceptable.

Host access implementations should own mount discovery, filesystem permissions, OS sync, safe eject, and host-specific failure handling.

### Payload And Release Layer

Responsibilities:

- Fetch KOReader release metadata.
- Select the correct artifact for the target.
- Fetch launch integration payloads from trusted sources.
- Validate downloads by checksum or published release metadata where available.
- Extract payloads into staging areas before writes.
- Keep release selection deterministic and inspectable.

### Persistence Layer

Use SQLite for local state that needs transactions, queryability, and auditability.

Store:

- Known devices and observed identifiers.
- Backup manifests.
- File hashes and modified timestamps.
- Operation logs.
- Release metadata cache.
- User acknowledgements for specific write-risk categories.

Do not store secrets unless a future feature explicitly requires it and has a defined encryption and retention plan.

### Safety Layer

Responsibilities:

- Dry-run previews.
- Backup-before-write enforcement.
- Write confirmation gates.
- Path containment checks.
- Hash and timestamp verification.
- Atomic staging where possible.
- OS sync and safe eject.
- Plain-language recovery instructions.

Safety behavior should be shared across all target devices.

## Initial Technical Stack

- Headless, testable core modules for host access, device targets, workflows, payload handling, persistence, and safety behavior.
- SQLite through the selected core implementation stack for manifests, device records, and logs.
- Diagnostic or command-line entry points as needed to validate risky workflows before production frontend work.
- Nix development environment for reproducible local development and Linux build consistency.
- GitHub REST API or equivalent release endpoint access for KOReader release fetching.
- Production frontend framework to be selected later by `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation`.

## Expansion Rule

Every new device target or host platform should be added by implementing an existing interface or extending a capability contract. If adding a platform requires rewriting core workflow logic, the architecture has failed its goal.
