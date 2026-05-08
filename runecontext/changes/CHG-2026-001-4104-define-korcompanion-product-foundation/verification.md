# Verification

## Planned Checks
- Read `proposal.md`, `design.md`, and `verification.md` together for internal consistency around trust boundaries, non-goals, success criteria, and review scope.
- Confirm the change stays documentation-first and that any repo-facing edits are limited to product-positioning, trust-boundary, and contributor-verification language consistent with this change.
- Run the repository's fast verification surface for this documentation-heavy change.

## Repository Verification Flow
- Use `just ci-fast` as the expected repository verification command for this change group.
- If `just ci-fast` is unavailable in the current repo state, record that assumption and use the closest documented repository verification command without expanding scope.

## Close Gate
- Do not close this change until the documentation is internally consistent and the repository fast verification flow has been run, or any repo-state limitation has been explicitly recorded.
