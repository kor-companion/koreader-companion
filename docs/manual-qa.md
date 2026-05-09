# Manual QA Matrix

Use this short matrix when manually checking the current headless foundation.
It complements `just ci-fast`; it does not replace automated verification.

## Repeatable Manual Checks

| Area | Setup | Command | Expected result |
| --- | --- | --- | --- |
| Repository baseline | None | `just ci-fast` | Source-size check passes, `nix flake check` evaluates, current-system flake checks are built explicitly, and `cargo test --workspace` passes. |
| Flake wiring | None | `nix flake check` | Flake outputs evaluate cleanly. On a single host it may still print `running 0 flake checks...`; use `just nix-checks` for the actual current-system check derivations. |
| Foundation report | None | `cargo run -p kc-diagnostic -- foundation` | Prints the current host, known host adapters, and the Kobo USB mass-storage target. |
| Probe a synthetic Kobo-like mount | `tmpdir="$(mktemp -d)" && mkdir -p "$tmpdir/kobo/.kobo/Kobo" "$tmpdir/kobo/.adds" && touch "$tmpdir/kobo/.kobo/Kobo/Kobo eReader.conf"` | `cargo run -p kc-diagnostic -- probe "$tmpdir/kobo"` | Reports `current readiness: ready`, no device blockers, scoped install/backup targets, and sync/eject automation readiness guidance. |
| Probe an incomplete mount | `tmpdir="$(mktemp -d)" && mkdir -p "$tmpdir/missing"` | `cargo run -p kc-diagnostic -- probe "$tmpdir/missing"` | Reports `current readiness: blocked` with blockers for missing Kobo markers such as `.kobo` or `Kobo eReader.conf`. |

## Notes

- `kc-diagnostic probe` is intentionally a manual path probe. It does not claim the path is a removable device mount.
- `just nix-checks` is the practical Nix verification command for contributors because it pairs `nix flake check` with explicit builds of the local system's flake check derivations.
