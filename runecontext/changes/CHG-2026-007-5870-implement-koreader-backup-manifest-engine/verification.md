# Verification

## Planned Checks
- Run schema tests for backup sets, files, hashes, timestamps, device metadata, operation links, and schema versioning.
- Run fixture backup tests for KOReader settings, `.sdr` directories, `metadata.lua`, user assets, launcher config, and `Kobo eReader.conf`.
- Run tests proving books and reinstallable application binaries are excluded by default unless explicitly included by a profile.
- Run path-containment tests for source and destination paths.
- Run hash verification tests, skipped-file handling tests, cancellation tests, and partial-backup recovery tests.
- Run human-readable summary tests proving summary content is generated and remains consistent with the manifest.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after backup sets are queryable, hash-verifiable, inspectable without the app database, and usable by CHG-008 restore workflows.
