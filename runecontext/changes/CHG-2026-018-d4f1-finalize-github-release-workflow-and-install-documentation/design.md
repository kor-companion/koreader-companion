# Design

## Overview
Finalize the MVP release path by adding a GitHub release workflow that consumes unsigned Nix release artifacts, signs them, publishes them to GitHub Releases, and documents installation in `README.md`.

## Shape Rationale
- Minimum mode is sufficient for the current size and risk signal.

## Workflow Requirements

The release workflow should be explicit and reviewable.

Expected behavior:

- Trigger on version tags or an explicit release workflow dispatch.
- Check out the repository at the release ref.
- Install or enable Nix.
- Build the unsigned release artifact flake output, such as `packages.release-artifacts` or the project-specific equivalent.
- Treat Nix outputs as unsigned build products.
- Sign artifacts after Nix generation using GitHub Actions secrets or another approved release secret mechanism.
- Generate checksums or a release manifest for signed artifacts.
- Upload signed artifacts, checksums, and manifests to the GitHub Release.
- Fail closed if signing material is unavailable for a real release.

The workflow should not recreate packaging steps that are already defined in Nix. GitHub Actions should orchestrate, sign, and publish.

## README Install Section

`README.md` should include an end-user installation section.

It should cover:

- Where to find the latest release.
- Which asset to download for each supported desktop platform.
- How to verify or at least inspect checksums/signatures once the release process supports them.
- Basic install or run instructions for Linux, macOS, and Windows as supported by the MVP.
- A note that developer builds use `nix develop` and are not the same as signed release artifacts.
- Minimum warning that the application manages e-reader files and users should follow in-app backup and confirmation prompts.

## Signing Boundary

Nix remains responsible for reproducible unsigned artifacts. The GitHub release workflow is responsible for signing and publishing.

The repository must not contain signing keys, release certificates, tokens, or secrets. Secret names and required setup should be documented for maintainers without exposing secret values.

## Acceptance Criteria

- A maintainer can create a release without manually rebuilding artifacts outside the workflow.
- The workflow consumes Nix release outputs rather than duplicating package construction.
- Release assets are signed or the workflow fails for a production release.
- Published assets include checksums or a release manifest.
- README install instructions are clear enough for a non-developer user to find, download, and install the app.
- Developer setup instructions remain separate from end-user installation instructions.

## Feature Intake Checklist

- User story: a user can install the released MVP without using the development shell.
- Acceptance criteria: release artifacts are built by Nix, signed by GitHub Actions, published to GitHub Releases, and documented in `README.md`.
- UX/API behavior: no in-app behavior is introduced, but user-facing install documentation is required.
- Data model or migration impact: none.
- Failure modes and observability: releases fail if unsigned artifacts cannot be built or production signing cannot complete.
- Tests and rollout plan: verify workflow dry-runs where possible and perform a tagged release candidate.

## Ask More When

- Choosing exact signing tool or certificate authority.
- Adding app-store, notarization, or package-repository distribution.
- Supporting mobile release distribution.
