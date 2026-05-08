# Design

## Overview
Define durable product language for KORCompanion before implementation continues so vision, audience, MVP scope, trust boundaries, and review gates stay stable across follow-on changes.

## Shape Rationale
- Product-foundation mistakes would ripple into every host, device, and workflow change.
- This change makes the product's trust and safety posture explicit before implementation choices harden.

## Project Intake Checklist
- Mission and target users are stated in stable product language.
- Deployment, security, and trust constraints are explicit.
- MVP scope and non-goals are explicit.
- Success criteria are reviewable at the product-foundation level.
- Verification planning names the repository close gate for this documentation-first change.

## Ask More When
- The product is asked to cross an existing trust boundary.
- A proposed workflow would write to a device without an explicit confirmation step.
- A proposed feature would require hosting, cloud sync, account systems, or background remote control.
- A proposed change introduces exploit, jailbreak, firmware, or proprietary asset handling.
- Success criteria drift from product language into unapproved implementation detail.

## Security, Deployment, And Trust Constraints

- KORCompanion is a user-operated companion application, not a remote management service.
- The product foundation assumes local, user-initiated workflows on devices the user already owns or is authorized to manage.
- No jailbreak automation, exploit distribution, exploit guidance packaged as workflow steps, or device-compromise enablement is in scope.
- No proprietary firmware, vendor-private assets, bundled copyrighted ROMs, or redistributed closed device payloads are in scope.
- Unsafe device writes must never happen silently. Risk-bearing writes require explicit user confirmation with enough context to understand what will change.
- Trust is limited to transparent, reviewable workflows: detect, explain, confirm, act, verify, and report. Hidden background mutation is out of bounds.
- The product foundation should preserve legal and user-trust boundaries already described elsewhere in the project rather than reinterpret them here.

## Product Boundaries

- The MVP foundation is for a KOReader companion application that helps users understand supported workflows, safety expectations, and product scope before feature work expands.
- This change defines language for target audience, value, MVP boundaries, and risk posture; it does not define implementation architecture beyond what is already decided elsewhere.
- Future changes may refine supported hosts and devices, but they must stay inside the trust constraints defined here unless a later reviewed change updates them.

## Non-Goals

- Rewriting unrelated repo-wide planning documents outside the product-foundation surface.
- Selecting new implementation frameworks or delivery channels beyond current project decisions.
- Defining feature-by-feature UX copy, installer flows, or technical execution details for later implementation changes.
- Promising unsupported device modification paths, jailbreak assistance, exploit content, or firmware redistribution.
- Expanding scope into cloud accounts, telemetry-heavy product behavior, or unattended remote administration.

## Success Criteria

- The change leaves stable, reviewable language for KORCompanion's product vision and intended audience.
- The MVP scope is explicit enough that follow-on implementation changes can tell what belongs in scope versus out of scope.
- Trust boundaries are explicit enough that later changes can reject jailbreak automation, exploit distribution, proprietary assets, and unsafe unconfirmed writes without re-debating the baseline.
- Non-goals are explicit enough to prevent scope creep into unrelated deployment, cloud, or repo-marketing work.
- Verification expectations are explicit enough that a documentation-heavy change can be closed using the repository verification flow without inventing code-specific gates.

## Verification Planning

- Review this change for internal consistency across proposal, design, and verification documents.
- Confirm the wording stays at the product-foundation level and does not drift into speculative implementation detail.
- Confirm any repo-facing documentation changes stay limited to product-positioning, trust-boundary, and contributor-verification language that matches this change.
- Confirm the repository close gate named in `verification.md` matches the expected documentation-change flow for the repo.
