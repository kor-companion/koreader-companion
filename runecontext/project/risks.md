# Risk Register

This register captures the main risks that should shape implementation order and product claims.

## Product And Adoption Risks

### Misidentified Audience

Risk: Completely average e-reader users may not know KOReader exists or have a reason to install it.

Mitigation: Target motivated non-developers first and explain outcome-driven benefits such as better PDF, manga, sideloaded document, reading statistics, and backup workflows.

### Community Trust Gap

Risk: A new app that writes to e-reader storage may be distrusted by the communities most likely to recommend it.

Mitigation: Build in dry-runs, logs, backup-before-write behavior, public device matrices, clear recovery docs, and transparent source code.

### Overbroad First Release

Risk: Promising every device and host platform early makes the project appear speculative and increases delivery risk.

Mitigation: Ship a high-quality Kobo desktop vertical slice first, while documenting a capability-based architecture for expansion.

## Technical Risks

### General Lua Parser Complexity

Risk: KOReader settings files use Lua syntax that is difficult to parse and safely round-trip in any selected implementation stack. A malformed write could corrupt user configuration.

Mitigation: Treat Lua files as opaque backup/restore artifacts in MVP. Use narrow, tested transformations for known settings before considering a general parser.

### Android Mobile Host Storage Access

Risk: Android USB OTG and Storage Access Framework do not provide normal POSIX filesystem semantics. Hidden directories, traversal speed, write permissions, and URI-based access may block reliable mobile host support.

Mitigation: Defer Android mobile host support to a post-MVP research spike with real device testing before product commitments.

### Desktop Mount Detection Fragility

Risk: Linux, macOS, and Windows expose removable storage differently, and mount polling alone can miss devices or fail under permissions and sandboxing.

Mitigation: Put mount discovery behind host-specific adapters, support manual path selection where appropriate, validate sentinel files, and fail closed when a device is ambiguous.

### Safe Ejection And Data Integrity

Risk: Disconnecting an e-reader before writes flush can corrupt device databases or files.

Mitigation: Use OS sync or ejection APIs where possible, wait for flush completion, warn when safe eject cannot be confirmed, and keep operation logs.

### Release Payload Selection

Risk: Selecting the wrong KOReader artifact or launcher payload can break installation.

Mitigation: Implement target-aware release matching, validate artifacts, stage extraction before writes, and include dry-run summaries.

## Platform Risks

### Kindle Legal And Detection Risk

Risk: Kindle installation support can implicate jailbreak, anti-circumvention, vendor ToS, and unreliable unlock detection concerns.

Mitigation: Do not automate jailbreaking. Implement Kindle only when a concrete compatible-state detector exists and stop otherwise.

### reMarkable Firmware Instability

Risk: reMarkable firmware and package-manager compatibility can change over time.

Mitigation: Treat reMarkable as post-MVP research and require current compatibility validation before implementation.

### ADB Packaging And Permissions

Risk: ADB workflows vary by host, device authorization state, vendor firmware, and whether a system or bundled ADB binary is used.

Mitigation: Implement ADB behind a dedicated host capability, show authorization status clearly, and support system ADB before bundling binaries.

### Premature Frontend Framework Lock-In

Risk: Selecting Flutter, Qt, Tauri, Electron, native UI, or another frontend before validating host/device access can cause frontend assumptions to leak into safety-critical workflows.

Mitigation: Build and validate the headless foundation first. Defer frontend framework selection to a dedicated evaluation change after discovery, install, eject, backup, restore, and payload behavior are proven.
