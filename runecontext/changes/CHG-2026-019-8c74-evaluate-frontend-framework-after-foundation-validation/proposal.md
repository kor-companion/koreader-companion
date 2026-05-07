## Summary
Evaluate frontend framework after foundation validation

## Problem
The project needs a trustworthy desktop application, but the riskiest MVP work is not visual UI. It is host/device integration, safe filesystem behavior, payload validation, backup/restore correctness, and OS sync or safe eject behavior.

Choosing Flutter, Qt, Tauri, Electron, native UI, or another frontend before those risks are validated can cause the frontend framework to shape the core architecture prematurely.

## Proposed Change
After the headless foundation and risky Kobo desktop workflows are implemented and validated, run a frontend framework evaluation using real project constraints and prototype evidence.

The evaluation must compare at least:

- Flutter with Material 3.
- Qt/QML or Qt Widgets.
- Tauri with a native core.
- Electron with native modules or sidecars.
- A native-platform or hybrid approach if the validated core suggests it.

The evaluation should produce a recommendation, implementation plan, and any follow-up frontend implementation change needed before community beta packaging.

## Why Now
The current roadmap previously assumed Flutter early. The revised plan front-loads the risky backend/device work first so frontend selection is evidence-driven.

## Assumptions
- The maintainer has Flutter and Material 3 mobile experience, so Flutter remains a serious candidate.
- Flutter is not selected by default merely because of that experience.
- Future Android host support is post-MVP and blocked on USB OTG and Storage Access Framework research, so Android reuse is an input but not the primary MVP decision factor.
- The selected frontend must consume domain workflow state and must not own safety-critical filesystem or install logic.

## Out of Scope
- Production frontend implementation.
- Rewriting the headless core to fit a frontend framework.
- Store deployment.
- Mobile host implementation.

## Impact
This change preserves optionality while making frontend choice more rigorous. It also records current research so the team does not repeat the same framework analysis later.
