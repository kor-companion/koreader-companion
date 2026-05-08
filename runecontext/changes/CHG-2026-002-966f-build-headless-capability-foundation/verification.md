# Verification

- Run unit tests for capability contracts, workflow state, path containment, and plan/log attribution.
- Run persistence tests for SQLite manifests, operation logs, hashes, timestamps, and release metadata cache.
- Run fixture-based tests for supported and unsupported device roots.
- Run Rust build, test, format, and lint commands documented by CHG-017.
- Run host diagnostics for mount discovery and safe-eject support where representative hosts are available.
- Confirm no production frontend framework dependency is introduced by this change.
- Confirm host and device adapters can be tested through interfaces without hard-coded Kobo-only workflow branches.

## Planned Checks
- Define the repository verification commands before closing this change.

## Close Gate
Use the repository's standard verification flow before closing this change.
