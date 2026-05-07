# Legal And Trust Model

KORCompanion should behave as an interoperability and management utility. It should help users manage software and data on devices they own while avoiding exploit distribution, jailbreak automation, proprietary firmware distribution, and opaque risky writes.

## Hard Boundaries

- Do not bundle jailbreaks, exploits, bypass tools, or circumvention payloads.
- Do not automate jailbreaking or locked-bootloader bypass workflows.
- Do not host or distribute proprietary firmware, vendor binaries, or vendor-owned assets.
- Do not claim vendor endorsement.
- Do not write to a connected device before explicit user confirmation.
- Do not hide risk behind a generic disclaimer when the app can provide a concrete device-specific warning.

## Device Unlock Policy

For devices that require jailbreak, developer mode, or another unlocked state before KOReader can be installed, KORCompanion should:

- Detect the device and explain that additional manual steps are required.
- Check for a concrete compatible state only when reliable detection rules exist.
- Halt installation if the compatible state cannot be verified.
- Link to established community documentation rather than reproducing exploit instructions inside the app.
- Continue only for already-compatible devices and only after the user confirms the risk.

Kindle support must not be implemented until the project has documented, testable unlocked-state detection rules for each supported class of device or install path.

## User Acknowledgements

Before write operations, the UI should require acknowledgement of relevant risks:

- Vendor warranty and support may be affected.
- Vendor terms of service or EULA may be implicated.
- Device firmware updates can change compatibility.
- File operations can cause data loss if interrupted.
- User data should be backed up before installation, restore, or configuration changes.

Acknowledgements should be specific and contextual, not a one-time blanket warning that users ignore.

## Trust Features

Trust should come from product behavior, not only legal text.

- Dry-run every operation.
- Show planned reads and writes.
- Back up before writing.
- Keep action logs.
- Verify backup manifests.
- Provide rollback and recovery instructions.
- Publish supported device and firmware matrices.
- Publish known limitations.
- Use open-source licensing with clear warranty disclaimers.

## License Direction

The project should use a permissive open-source license with an explicit no-warranty clause, such as MIT or Apache-2.0. The final license choice should be made before public distribution and should align with dependency licensing.
