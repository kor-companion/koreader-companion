# Verification

## Planned Checks
- Run frontend unit tests for required screens or surfaces once the selected framework exists.
- Run integration tests proving device discovery, dry-run, install confirmation, backup, restore preview, operation logs, and eject status are rendered from core state.
- Run tests proving the frontend cannot execute write workflows without core confirmation gates.
- Run tests proving filesystem writes, path containment, release validation, backup/restore execution, and eject behavior remain in the Rust core.
- Run accessibility checks appropriate to the selected framework.
- Run packaging smoke tests for the supported MVP desktop hosts before CHG-009.
- Run the repository's Rust and frontend build, format, lint, and test commands.

## Close Gate
Close only after the MVP frontend shell is usable for the complete Kobo desktop flow and remains a presentation/control layer over the validated headless core.
