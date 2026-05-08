# Design

## Overview
Implement a deterministic Kobo install workflow that never writes outside the selected device root, never writes before confirmation, and never writes unvalidated payloads.

## Inputs

- Supported Kobo root and existing-state report from CHG-003.
- Validated staged KOReader Kobo payload from CHG-004.
- Safety and persistence services from CHG-002.
- User confirmation for the final write plan.

## Planned Writes

The install plan should list and log every write. Initial known paths include:

- `.adds/koreader` for the KOReader application payload.
- Launcher integration files for the selected launcher.
- `.kobo/Kobo/Kobo eReader.conf` when the `ExcludeSyncFolders` patch is required.
- Backups of every file that will be modified or replaced.
- Operation log entries in local persistence.

## Launcher Integration

The implementation must explicitly choose the MVP default launcher path before writes. Research notes:

- KFMon uses `.adds/kfmon/config/*.ini` entries and launcher icons such as `koreader.png` to trigger `/mnt/onboard/.adds/koreader/koreader.sh`.
- NickelMenu uses entries under `.adds/nm` and can spawn `/mnt/onboard/.adds/koreader/koreader.sh` from Nickel menus.
- Some launcher setup paths use `.kobo/KoboRoot.tgz`, which is consumed by the Kobo firmware on eject/reboot. Pending `KoboRoot.tgz` state must be detected and explained.

The chosen path should favor reliability, current upstream support, and reversibility. Existing supported launcher state may be reused when the dry-run can prove it targets the staged KOReader launch script.

## Kobo Configuration Patch

When required, patch `.kobo/Kobo/Kobo eReader.conf` idempotently:

- Preserve unrelated sections, keys, comments where practical, and line endings where feasible.
- Ensure `[FeatureSettings]` exists.
- Ensure `ExcludeSyncFolders=(\\.(?!kobo|adobe).+|([^.][^/]*/)+\\..+)` is present without duplicating conflicting values.
- Back up the original file before modifying it.
- Verify the modified file can be read back and contains the expected key before declaring success.

## Safety Behavior

- Use path containment for every source and destination.
- Back up before modifying or replacing files.
- Stage writes locally or in temporary paths where practical before final placement.
- Abort before writes if device identity, payload validation, launcher state, config patching, or free-space checks are ambiguous.
- Run OS sync and safe eject through the host adapter when supported, or warn when safe eject cannot be confirmed.
- Produce rollback guidance from the backup manifest and operation log.

## Firmware Update State

Official Kobo firmware updates may leave KOReader data in place while disrupting launcher integration. The workflow should detect installed KOReader with missing or broken launcher state and offer a launcher-repair plan instead of reinstalling user data blindly.

## Research Sources

- KOReader Kobo installation wiki: `https://github.com/koreader/koreader/wiki/Installation-on-Kobo-devices`.
- KFMon installer/config: `https://github.com/NiLuJe/kfmon`.
- NickelMenu documentation: `https://github.com/pgaskin/NickelMenu`.
