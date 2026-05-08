## Summary
Prepare MVP community beta and release packaging

## Problem
Package signed desktop builds, prepare onboarding and recovery documentation, and run a community beta focused on Kobo desktop install and backup workflows after the frontend approach has been selected and the beta-ready app surface exists.

## Proposed Change
Prepare the MVP community beta after the headless Kobo workflows and MVP frontend shell are implemented. This includes beta packaging, supported-device documentation, onboarding, recovery documentation, known limitations, and beta feedback expectations.

## Why Now
This is the transition from validated workflows to a user-facing beta. It should not begin until CHG-020 provides a beta-ready frontend shell over the headless core.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Frontend framework selection has completed.
- `CHG-2026-020-a91c-build-mvp-frontend-shell-over-headless-core` has produced the beta-ready app surface.

## Out of Scope
- Implementing headless workflows.
- Selecting or building the frontend shell.
- Final GitHub release signing workflow, which is handled by CHG-018.

## Impact
This change prepares the project for real user feedback without hiding device support limits or recovery responsibilities.
