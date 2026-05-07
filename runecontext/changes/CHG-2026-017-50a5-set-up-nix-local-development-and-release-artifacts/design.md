# Design

## Overview
Set up the repository around a Nix flake so contributors can enter a predictable local development shell before implementing the headless foundation and risky host/device workflows. The same flake should later expose unsigned release artifact builds that a GitHub release workflow can sign and publish.

## Shape Rationale
- Minimum mode is sufficient for the current size and risk signal.

## Dev Shell Goals

- Provide a single `nix develop` entrypoint.
- Include core development, test, formatting, linting, packaging-support, and diagnostic tooling where practical.
- Include native host/device diagnostic dependencies for Linux where practical.
- Include common developer tools for formatting, testing, linting, and packaging scripts.
- Keep dependency versions pinned through `flake.lock`.
- Use nixpkgs unstable if it is the most practical source for current core, diagnostic, and packaging dependencies.
- Document host prerequisites that Nix cannot fully abstract, especially for USB/device access testing.
- Avoid frontend-specific dependencies until `CHG-2026-019-8c74-evaluate-frontend-framework-after-foundation-validation` selects a frontend approach.

## Nix Layout

The root `flake.nix` should stay small and delegate meaningful logic to a root-level `nix/` directory.

Expected initial layout:

- `flake.nix` wires inputs, systems, formatter, dev shell, checks, and packages.
- `flake.lock` pins the selected inputs.
- `nix/dev-shell.nix` defines the contributor shell.
- `nix/checks.nix` defines flake checks as they become available.
- `nix/packages/` holds package and release-artifact builders.
- `nix/release/metadata.nix` holds release metadata when artifact generation needs stable names, versions, or manifest data.

This structure should be introduced early even if some files start small. The goal is to avoid reworking the flake when release automation is added.

## Release Artifact Direction

The flake should be designed so release packaging can be added as Nix outputs later.

Target direction:

- Nix builds unsigned project artifacts.
- Unsigned artifacts are deterministic enough to inspect and archive.
- Unsigned artifact outputs are exposed through flake packages, for example a future `packages.release-artifacts` output.
- Release metadata and artifact naming live under `nix/release/` or `nix/packages/` rather than inside GitHub workflow YAML.
- GitHub Actions invokes the Nix build outputs in a release workflow.
- Signing happens after Nix artifact generation inside the release workflow boundary.
- Signed assets are uploaded to GitHub Releases.

This keeps build reproducibility separate from signing authority. Nix should produce the bits; the release workflow should apply signatures and publish them.

## Initial Implementation Expectations

- Add `flake.nix` and `flake.lock`.
- Add a root-level `nix/` directory with separate files for the dev shell and future package/check/release logic.
- Provide at least one default development shell.
- Expose a formatter through the flake.
- Add a placeholder-friendly checks structure so future checks can be added without changing the flake shape.
- Prefer clear package names and minimal shell hooks over clever automation.
- Document usage in project or repository docs.
- Avoid committing signing keys, credentials, or release secrets.
- Do not require contributors to run release signing locally.

## Future Release Outputs

Later changes may add flake outputs for:

- Linux desktop bundles.
- macOS app artifacts where feasible from appropriate runners.
- Windows artifacts where feasible from appropriate runners or separate packaging lanes.
- Checksums for unsigned artifacts.
- Release manifests consumed by GitHub Actions.
- A release metadata module that centralizes version, artifact names, and expected output paths.

## Feature Intake Checklist

- User story: a contributor can clone the repository, enter `nix develop`, and use documented commands to work on the app.
- Acceptance criteria: the dev shell provides the expected core, diagnostic, test, and packaging-support tools without locking in a production frontend framework.
- UX/API behavior: no runtime product UI behavior is introduced by this change.
- Data model or migration impact: none.
- Failure modes and observability: missing host prerequisites should be documented clearly.
- Tests and rollout plan: validate the flake, enter the dev shell, and run the available repository checks.

## Ask More When

- Deciding exact release signing technology.
- Adding platform-specific signing, notarization, or installer generation.
- Adding GitHub secrets or release credentials.
- Adding mobile build support.
