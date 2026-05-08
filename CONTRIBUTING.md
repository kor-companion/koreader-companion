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

The planned canonical local workflow uses Nix once the development environment
change is implemented. Until then, keep changes focused, documented, and
aligned with the RuneContext roadmap and specs.

## Submitting a Pull Request

- Keep changes focused and well-described.
- Ensure your commits are signed off with `git commit -s`.
- Do not include secrets, credentials, or private keys.
- Update relevant planning/docs when the change affects project direction.
- Follow the active roadmap and change documents under `runecontext/`.

If you are unsure about a design direction, open an issue or discussion first.
