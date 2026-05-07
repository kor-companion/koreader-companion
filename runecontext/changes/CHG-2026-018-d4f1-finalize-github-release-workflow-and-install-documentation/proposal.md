## Summary
Finalize GitHub release workflow and install documentation.

## Problem
The MVP needs a complete public release path after the app has been built, packaged, and beta validated. Earlier Nix work prepares unsigned artifacts, but users still need a GitHub release workflow that signs and publishes those artifacts and README installation instructions that explain how to get and install the application.

## Proposed Change
Add the final MVP release change.

The change should provide:

- A GitHub release workflow that builds the unsigned Nix release artifacts from the flake outputs.
- A signing step that signs release artifacts after Nix builds them.
- Checksums or a release manifest for published assets.
- GitHub Release asset upload for signed artifacts and supporting verification files.
- A README installation section that explains how normal users install the released application.
- Clear distinction between released signed artifacts and developer-only local builds.

This change should consume the Nix release-artifact shape created by the local development setup change rather than duplicating packaging logic in workflow YAML.

## Why Now
This belongs at the end of the MVP because the release workflow should publish the validated MVP application, not drive early implementation. It closes the MVP by making the app installable by users outside the development environment.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Unsigned Nix artifacts already exist or are shaped by the first MVP development-environment change.
- The release workflow can use GitHub-hosted signing secrets or another approved GitHub Actions secret mechanism.
- README install instructions should target end users first and contributors second.

## Out of Scope
- Designing the application packaging format from scratch outside the Nix artifact path.
- Replacing the Nix release artifact outputs with bespoke GitHub Actions packaging logic.
- Publishing to app stores or package repositories.
- Mobile release distribution.
- Automating vendor-specific device installation outside the app itself.

## Impact
The MVP gains a complete release path: Nix creates unsigned artifacts, GitHub Actions signs and publishes them, and users can follow README instructions to install the application.
