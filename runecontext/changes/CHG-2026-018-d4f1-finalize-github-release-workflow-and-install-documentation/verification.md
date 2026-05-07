# Verification

## Planned Checks
- Run `nix flake check`.
- Build the unsigned release artifact flake output locally or in CI.
- Run the GitHub release workflow in a release-candidate path or dry-run path where possible.
- Confirm the workflow signs artifacts after Nix generation.
- Confirm the workflow publishes signed artifacts and checksums or a release manifest.
- Confirm production release workflow fails closed if signing secrets are missing.
- Confirm `README.md` contains end-user install instructions and separates them from developer setup.
- Confirm no signing keys, certificates, tokens, or release secrets are committed.

## Close Gate
Close only after a maintainer can publish signed MVP release assets from unsigned Nix artifacts and a user can install the app by following `README.md`.
