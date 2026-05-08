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

- Rust headless core modules for host access, device targets, workflows, payload handling, persistence, and safety behavior.
- SQLite through Rust bindings for manifests, device records, release metadata cache, backup records, and logs.
- Diagnostic or command-line entry points as needed to validate risky workflows before production frontend work.
- Nix development environment for reproducible local development and Linux build consistency.
- GitHub REST API or equivalent release endpoint access for KOReader release fetching.
- Production frontend framework to be selected later by `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation`.

Rust is the default headless-core language unless Kobo research or implementation evidence uncovers a strong reason to change course. This gives the project strong filesystem safety, explicit error handling, mature cross-platform packaging support, good SQLite options, and multiple future frontend integration paths without requiring the production UI to be Rust-based.

## Module Model

The core should be organized as shared logic plus host and device modules. Naming can evolve during implementation, but the architectural shape should remain stable:

- Shared core: capability contracts, workflow state machines, dry-run plans, safety gates, path containment, operation logs, and user-facing domain events.
- Persistence module: SQLite schemas, migrations, manifest storage, release metadata cache, device records, and operation logs.
- Payload module: KOReader release lookup, artifact selection, checksum validation, staging, and extraction.
- Host modules: Linux, macOS, Windows, and future Android or iOS host adapters that implement mount discovery, file access, permission checks, sync, and safe eject where supported.
- Device modules: Kobo first, then PocketBook, Kindle-compatible unlocked states, Android/ADB devices, reMarkable/SSH targets, and future targets as separate implementations.
- Diagnostic entrypoints: command-line or test harness surfaces that exercise workflows before production frontend work.
- Frontend integration: a later UI consumes domain workflow state through the integration shape chosen after frontend evaluation, such as direct library calls, IPC, local service boundaries, or FFI.

Shared workflows must depend on host and device capabilities rather than concrete platform names. Adding a new host or device should mean adding a module and tests, not rewriting install, backup, restore, safety, or release-selection logic.

## Expansion Rule

Every new device target or host platform should be added by implementing an existing interface or extending a capability contract. If adding a platform requires rewriting core workflow logic, the architecture has failed its goal.
