# Verification

## Planned Checks
- Run unit tests for Kobo asset matching, including stable releases, missing assets, duplicate matches, prereleases, and naming changes.
- Run checksum validation tests for GitHub asset digests and deterministic fallback checksum parsing.
- Run cache tests for release metadata persistence, refresh behavior, rate-limit handling, and stale cache warnings.
- Run archive validation tests for valid Kobo payloads, missing `koreader/`, missing launch script, path traversal entries, and malformed archives.
- Run local-artifact tests that enforce the same validation rules as downloaded artifacts.
- Run dry-run integration tests showing selected version, asset, digest source, staged paths, and blocked states without device writes.
- Run Rust build, format, lint, and test commands defined by the repository.

## Close Gate
Close only after validated staged payloads can be consumed by CHG-005 without reimplementing release selection logic.
