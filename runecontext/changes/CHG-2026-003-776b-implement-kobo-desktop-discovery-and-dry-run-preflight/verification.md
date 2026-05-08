# Verification

## Planned Checks
- Run unit tests for Kobo sentinel validation, including valid roots, missing `.kobo/`, missing `Kobo eReader.conf`, wrong selected paths, and nested/path-traversal attempts.
- Run host-adapter fixture tests for Linux, macOS, and Windows mount metadata, including `KOBOeReader` label detection and manual path fallback.
- Run read-only dry-run tests that confirm no files are written during discovery or preflight.
- Run fixture tests for existing KOReader, KFMon, NickelMenu, launcher icon, and pending `KoboRoot.tgz` detection.
- Run failure tests for unreadable roots, permission errors, insufficient free-space signals, and ambiguous Kobo-like paths.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after fixture coverage proves discovery fails closed and the dry-run plan is detailed enough for CHG-004, CHG-005, and CHG-007 to consume.
