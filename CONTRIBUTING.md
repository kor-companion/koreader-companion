# Contributing to KOReader Companion

Thanks for your interest in contributing.

## License

KOReader Companion is licensed under Apache-2.0. By contributing, you agree
that your contributions will be licensed under the same license.

## Developer Certificate of Origin (DCO) (Required)

KOReader Companion uses the Developer Certificate of Origin (DCO) instead of a
CLA. Every commit in a pull request must include a `Signed-off-by:` line.

To add it when committing:

```sh
git commit -s
```

Example sign-off line:

```
Signed-off-by: Jane Smith <jane.smith@example.com>
```

The DCO text is in `DCO` and at https://developercertificate.org/.

### Fixing missing sign-offs

If you forgot to sign off:

- Last commit only:

```sh
git commit --amend -s
```

- Multiple commits on your branch (one common approach):

```sh
git rebase --signoff origin/main
```

## DCO Enforcement

We enforce DCO on pull requests using the GitHub-side DCO check.
PRs should not be merged unless all commits are signed off.

Maintainers should:

- Install the DCO GitHub App: https://github.com/apps/dco
- Require the DCO check in branch protection rules
- Enable GitHub's "Require contributors to sign off on web-based commits"

## Project Status

The repository is currently in the planning and architecture stage. The main
project context, roadmap, and change tracking live under `runecontext/`.

## Development

The canonical contributor workflow uses the Nix flake dev shell that is already
checked into this repository.

Start with:

```sh
nix develop
```

See `docs/development.md` for the current local workflow, host prerequisites,
flake validation commands, and the Rust-oriented commands that become active
once the headless workspace lands.

Important scope notes:

- Nix is the primary path for contributor setup and future CI/release work.
- A non-Nix Rust path may be practical for some contributors later, but the
  repository docs treat it as secondary.
- Optional `direnv` usage should stay limited to local shell activation. Do not
  put secrets, signing material, or credentials in `.envrc`.
- Frontend-specific toolchains are intentionally deferred until the roadmap's
  frontend evaluation change.
- End-user installation and signed release artifacts are not part of the current
  contributor setup; those docs arrive in a later release-focused change.

## Submitting a Pull Request

- Keep changes focused and well-described.
- Ensure your commits are signed off with `git commit -s`.
- Do not include secrets, credentials, or private keys.
- Update relevant planning/docs when the change affects project direction.
- Follow the active roadmap and change documents under `runecontext/`.

If you are unsure about a design direction, open an issue or discussion first.
