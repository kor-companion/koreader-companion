# Verification

## Planned Checks
- Run `nix flake check` when the flake exposes checks.
- Run `nix develop` and confirm the shell enters successfully.
- Run `nix flake show` and confirm the expected formatter, dev shell, checks, and package shape are visible.
- Run documented core, diagnostic, formatting, linting, and test commands as they become available.
- Confirm Flutter, Dart, Qt, Tauri, Electron, or other frontend-specific dependencies are not required before frontend evaluation selects them.
- Confirm the root `flake.nix` delegates meaningful logic to the root-level `nix/` folder.
- Run the repository's formatting, linting, test, and build commands once they exist.
- Confirm release-signing secrets or credentials are not committed.
- Confirm any unsigned artifact outputs are clearly named as unsigned.

## Close Gate
Close only after the local dev shell is documented, reproducible, and usable as the required starting point for subsequent MVP implementation changes.
