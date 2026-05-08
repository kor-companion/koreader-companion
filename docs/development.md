# Development

This repository now provides a Nix flake as the canonical contributor setup.
Use it for local development, flake validation, and future CI-aligned work.

This is developer setup only. Signed release artifacts and end-user install
instructions are planned for a later release change.

## Current Status

- The Nix flake and dev shell are available now.
- The Rust headless workspace is planned in
  `CHG-2026-002-966f-build-headless-capability-foundation` and does not exist
  yet.
- Frontend-specific toolchains are intentionally deferred until the roadmap's
  frontend evaluation work selects one.

## Enter the Dev Shell

```sh
nix develop
```

The shell currently prepares the shared Rust-oriented toolchain and basic host
diagnostic utilities used by upcoming headless and device workflow changes.

Optional `direnv` users can use the tracked `.envrc` in the repository.
Keep that file limited to shell activation only; do not add secrets,
credentials, or signing material to it.

## Useful Nix Commands

Inspect flake outputs:

```sh
nix flake show
```

Validate the flake definition and checks:

```sh
nix flake check
```

Format Nix files through the flake formatter:

```sh
nix fmt
```

Update pinned dependencies when maintainers intentionally refresh the Nix input:

```sh
nix flake update
```

Review the resulting `flake.lock` changes before opening a pull request.

## Common Development Commands

These commands describe the expected local workflow once the Rust workspace
lands. Until then, they are placeholders for the upcoming headless foundation.

Inside `nix develop`, contributors are expected to use commands such as:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
cargo nextest run --workspace
cargo audit
cargo deny check
```

Likely future validation commands for the headless foundation include:

```sh
cargo build --workspace
cargo test --workspace
just ci
```

Use the Nix commands above even before the Rust workspace exists so you can
format Nix files and verify the development environment itself.

## Non-Nix Rust Setup

Nix is the canonical path. If you cannot use Nix, expect to install a standard
Rust toolchain plus any required native dependencies yourself. That path is
secondary and may lag behind the Nix environment during early foundation work.

## Host Prerequisites Nix Does Not Fully Abstract

Some host/device concerns still depend on your operating system and local
permissions.

### Linux

- USB access may require the correct user groups, udev/device permissions, or
  elevated privileges depending on the workflow being implemented.
- Mounted e-reader volumes must be readable and, for future install/backup
  workflows, writable by your user account.
- Useful diagnostics from the dev shell include `lsusb`, `lsblk`, `blkid`, and
  `lsof`.

### macOS and Other Hosts

- Device visibility, removable-volume permissions, and safe eject behavior still
  depend on host OS policies and user permissions.
- Nix can provide tooling, but it cannot bypass host security prompts or grant
  filesystem/device access automatically.

## Scope Boundaries

- Contributor setup is documented here.
- End-user installation guidance will be added later alongside signed release
  artifacts and GitHub Releases documentation.
- Frontend framework setup is intentionally absent until that decision is made.
