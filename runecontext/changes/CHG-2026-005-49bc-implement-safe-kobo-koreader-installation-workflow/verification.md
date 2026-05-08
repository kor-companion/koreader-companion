# Verification

## Planned Checks
- Run fixture-based install-plan tests for fresh install, existing KOReader, existing supported launcher, pending `KoboRoot.tgz`, missing config, and broken launcher state.
- Run path-containment tests for every planned source and destination, including archive entries with traversal attempts.
- Run backup-before-write tests proving affected files are copied and recorded before modification.
- Run `Kobo eReader.conf` patch tests for missing `[FeatureSettings]`, existing `ExcludeSyncFolders`, conflicting values, comments, line endings, and idempotent repeat runs.
- Run launcher integration tests for the selected launcher path and fixture tests for any supported reuse path.
- Run failure tests for insufficient space, read-only mount, failed backup, failed copy, failed config verification, failed sync/eject, and interrupted operation.
- Run operation-log tests proving every write maps to a plan item and rollback guidance.
- Run manual validation with at least one representative Kobo device or an explicitly documented hardware substitute before claiming public support.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after install writes are guarded by dry-run, confirmation, backup-before-write, verification, logs, and fail-closed error handling.
