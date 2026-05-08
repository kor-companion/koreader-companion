# Verification

- Run unit tests for capability contracts, workflow state, path containment, protected-write and rollback-guidance foundations, and plan/log attribution.
- Run persistence tests for SQLite manifests, operation logs, address/target encoding, hashes, timestamps, and release metadata cache.
- Run fixture-based tests for supported and unsupported device roots, including symlink-safe readiness behavior.
- Run repository verification through `just ci-fast`, including the source-size policy, `nix flake check`, and `cargo test --workspace`.
- Run targeted Rust crate tests while implementing the foundation modules.
- Run host diagnostics for capability reporting, manual path validation, and safe-eject readiness/guidance reporting.
- Confirm no production frontend framework dependency is introduced by this change.
- Confirm host and device adapters can be tested through interfaces without hard-coded Kobo-only workflow branches.
- Confirm the workspace is organized into logical submodules and that oversized files are surfaced by the repository source-size check.

## Checks Run
- `cargo fmt --all`
- `cargo test -p kc-domain`
- `cargo test -p kc-persistence -p kc-payload`
- `cargo test -p kc-host -p kc-device -p kc-diagnostic`
- `cargo test --workspace`
- `cargo run -p kc-diagnostic -- foundation`
- `just ci-fast`

## Close Gate
Use the repository's standard verification flow before closing this change.

## Residual Notes
- The foundation now includes honest sync/eject readiness and guidance boundaries, but not production-grade automated host sync/eject execution.
- The foundation now includes stronger path containment and manifest validation, but fd/openat-style hardening for actual write workflows remains follow-on work when install/backup/restore execution is implemented.
