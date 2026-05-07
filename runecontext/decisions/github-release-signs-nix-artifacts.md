---
schema_version: 1
id: github-release-signs-nix-artifacts
title: GitHub Release Signs Nix Artifacts
originating_changes:
  - CHG-2026-018-d4f1-finalize-github-release-workflow-and-install-documentation
related_changes:
  - CHG-2026-017-50a5-set-up-nix-local-development-and-release-artifacts
---

# GitHub Release Signs Nix Artifacts

## Decision

The GitHub release workflow should consume unsigned artifacts built by Nix, sign them in the workflow boundary, and publish signed assets plus checksums or manifests to GitHub Releases.

## Rationale

Nix should own deterministic artifact construction. GitHub Actions should own release orchestration, signing, and publication because that is where release credentials and GitHub release permissions belong. This preserves the build boundary established by the development and release Nix setup.

## Consequences

- Release packaging logic should remain in `flake.nix` and `nix/` modules.
- GitHub workflow YAML should not duplicate artifact construction.
- Signing secrets must live in GitHub Actions secrets or another approved release secret store.
- Production releases should fail if signing cannot complete.
- `README.md` must explain installation from signed GitHub Release assets.
