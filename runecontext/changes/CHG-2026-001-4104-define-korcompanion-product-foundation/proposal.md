## Summary
Define the stable product foundation for KORCompanion.

## Problem
The change folder still describes the product in placeholder terms, even though nearby project documents already establish a narrower and more specific direction. Follow-on work needs change-local language that clearly states the mission, target users, value proposition, and MVP boundaries for a KOReader companion application without forcing reviewers to infer intent from repo-wide docs.

## Proposed Change
Replace the skeletal change-local proposal and design with a concise product foundation for KORCompanion that:

- states the product mission as a safe, approachable KOReader companion for device owners,
- identifies the initial audience as motivated Kobo users and KOReader users operating from desktop hosts,
- defines the value proposition around safer installation, backup, restore, and device-management workflows,
- records the MVP boundary as Kobo devices from Linux, macOS, and Windows hosts first, and
- makes explicit that jailbreak automation, premature frontend commitments, and broader device promises are out of scope for this foundation change.

## Why Now
This is the product-foundation step immediately before headless-core work. Future changes need stable product language so implementation decisions, roadmap sequencing, and safety claims stay aligned around the same narrow MVP instead of drifting toward unsupported devices, unsafe automation, or frontend-first assumptions.

## Assumptions
- No selectable standards are defined in the project yet; the Applicable Standards section is rendered as N/A.
- Existing project docs already capture the richer product direction, and this change should align to them rather than redefine them.
- The first production-grade slice remains Kobo desktop first, with capability-based expansion later.

## Out of Scope
- Implementation code, device support, or frontend work.
- Repo-wide edits to vision, scope, legal, or risk documents.
- Any commitment to Kindle, reMarkable, Android host support, or other post-MVP targets.
- Jailbreak or exploit automation.

## Impact
The change makes the product intent reviewable in the change folder itself. That gives upcoming architecture and implementation work a stable reference for who the MVP serves, what user outcomes matter, and which boundaries must not be crossed.
