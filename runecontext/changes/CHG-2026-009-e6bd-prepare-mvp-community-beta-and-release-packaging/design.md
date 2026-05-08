# Design

## Overview
Prepare a community beta for the complete Kobo desktop MVP after the beta-ready frontend shell exists. This change should package the app for beta users, document support boundaries, and define feedback and recovery channels.

## Beta Requirements

- The MVP app surface from CHG-020 supports discovery, dry-run, install, backup, restore preview, logs, and safe-eject status.
- Supported host platforms and known limitations are documented.
- Supported Kobo device and firmware notes are documented, including launcher and firmware-update caveats.
- Recovery documentation explains backup locations, operation logs, rollback guidance, and how to report failures.
- Beta onboarding explains that the app writes to e-reader storage and users must follow confirmation prompts.

## Packaging Direction

- Produce beta artifacts for the selected MVP desktop platforms.
- Keep beta packaging distinct from final signed GitHub release automation in CHG-018.
- Document whether beta artifacts are signed, unsigned, notarized, or otherwise limited.
- Include checksums where practical.

## Feedback And Support

- Provide issue templates or reporting guidance for failed detection, failed install, failed eject, restore problems, and device/firmware compatibility.
- Ask beta users to include operation logs where safe.
- Keep support claims narrow: Kobo desktop MVP only.
