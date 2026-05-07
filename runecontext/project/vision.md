# KORCompanion Vision

KORCompanion is a standalone companion application for KOReader users and KOReader-curious e-reader owners. Its long-term purpose is to make KOReader installation, backup, restore, configuration, and device management approachable, safe, and repeatable across e-ink devices.

The product should eventually be all of these things:

- A safe installer for KOReader and required launch integrations.
- A backup and restore tool for KOReader settings, reading state, highlights, and book metadata.
- A full e-reader management suite for advanced KOReader operations across multiple devices.

The first production-grade vertical slice is intentionally narrower: Kobo target devices managed from desktop hosts. This narrow MVP is not a retreat from the larger vision. It is the first credible path to a trusted product because Kobo exposes a USB mass storage workflow, has a well-understood KOReader installation path, and avoids jailbreak automation.

## Product Promise

KORCompanion should let a motivated non-developer improve and protect their e-reader without understanding hidden folders, launcher payloads, shell commands, community forum archaeology, or fragile configuration edits.

The app should make these outcomes clear to users:

- Install KOReader on supported devices with clear preflight checks.
- Improve workflows for PDFs, manga, comics, sideloaded documents, file browsing, statistics, and advanced reading customization.
- Back up reading state before firmware updates, factory resets, device replacement, or experimentation.
- Restore selected KOReader data with confidence about what will change.
- Understand when a device is unsupported, locked, risky, or requires manual community steps.

## Audience

The target user is not every e-reader owner. The initial audience is normal but motivated users who have a reason to seek better reading software or safer device management.

Primary segments:

- KOReader-curious Kobo owners who want better PDF, manga, comic, sideloaded document, or advanced reading workflows but are nervous about manual installation.
- Existing KOReader users who want reliable backups before firmware updates, factory resets, device migrations, or experiments.
- Multi-device readers who want repeatable installs, restore flows, and eventually cross-device management.
- Community helpers who want a safer tool to recommend instead of repeating complex manual instructions.

Secondary segments:

- PocketBook, Kindle, reMarkable, Onyx Boox, and Android-tablet users once the corresponding device workflows are proven.
- Technical users who still prefer a transparent GUI with dry-run plans, action logs, checksums, and rollback guidance.

## Adoption Strategy

Lowering technical skill requirements can expand KOReader adoption, but the app must solve awareness, trust, and reversibility.

- Awareness: position the product around reader outcomes, not only around KOReader installation.
- Trust: show every planned write before it happens, keep logs, back up before writes, and avoid opaque automation.
- Reversibility: provide pre-install backups, restore flows, rollback guidance, and safe ejection.
- Community credibility: publish transparent behavior, supported device matrices, known limitations, and recovery docs so the KOReader and e-reader communities can inspect and recommend the tool.

## Product Positioning

Use broad project naming for the long-term vision, but initial public positioning should be explicit about support level.

- Project name: KORCompanion.
- MVP positioning: Kobo desktop companion for KOReader installation and backup.
- Long-term positioning: cross-device companion for KOReader installation, backup, restore, configuration, and management.

## Non-Negotiables

- No jailbreak or exploit automation.
- No bundled proprietary firmware or vendor-owned assets.
- No writes without explicit user confirmation.
- No installation without a preflight plan.
- No destructive restore without preview and backup guidance.
- No general Lua config editing in MVP unless it is replaced by narrow, tested transformations for known settings.
- No Android mobile host support until USB OTG and Storage Access Framework constraints are validated with real devices.
- No production frontend framework lock-in until the headless foundation and risky Kobo desktop workflows are validated.
