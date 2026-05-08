# Verification

## Planned Checks
- Run manifest verification tests for valid backups, missing files, changed hashes, incompatible schema versions, and inconsistent summary files.
- Run restore-plan tests for selected settings, `.sdr` directories, `metadata.lua`, launcher/config rollback files, and skipped out-of-scope books.
- Run destination path-containment tests and incompatible-destination tests.
- Run backup-before-restore tests proving overwritten files are backed up and linked to operation logs.
- Run failure tests for read-only destinations, insufficient space, interrupted restore, hash mismatch, and missing rollback backups.
- Run opaque Lua restore tests proving files are restored or skipped as whole artifacts, not parsed or merged.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after restore operations are previewed, verified, backed up, logged, and fail closed on integrity or containment problems.
