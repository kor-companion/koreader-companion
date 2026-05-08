# Verification

## Planned Checks
- Run operation-log tests proving every write maps to a plan item, confirmation, backup record, and outcome.
- Run rollback guidance tests for successful install, failed install before writes, partial write failure, failed config patch, failed restore, and missing backup files.
- Run confirmation-gate tests proving write workflows cannot execute without the required acknowledgement.
- Run host-adapter tests for successful eject, sync-only, unsupported eject, failed eject, and user-cancelled eject states.
- Run persistence tests for log durability, schema migration behavior, and querying logs by device, operation, and backup set.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after safety state is durable, user-readable, and reusable by install, backup, restore, and future configuration workflows.
