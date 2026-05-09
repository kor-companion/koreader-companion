---
schema_version: 1
id: development/source-size-policy
title: Source Size Policy
status: active
tags:
  - development
  - reviewability
  - source-quality
  - rust
---

# Source Size Policy

## Intent

Keep implementation files small enough to review safely as host, device, workflow, and safety logic grow.

## Requirements

- Checked-in Rust, Python, shell, Nix, and `Justfile` sources must stay within the repository source-size limit unless an explicit allowlisted exception is added with a short reason.
- Contributors should split oversized files into logical modules before they become difficult to review.
- Generated output, build directories, and similarly non-source trees should remain outside the enforced source-size scope.
- Fast repository verification must include the source-size check so the policy is enforced continuously rather than treated as optional guidance.

## Rationale

This repository is building safety-sensitive host, device, and filesystem workflow code. Smaller files reduce review risk, make architectural boundaries clearer, and help prevent core logic from collapsing into hard-to-audit monoliths.

## Implementation Notes

- The current enforcement entrypoint is `python3 scripts/check-source-size.py`.
- `just ci-fast` should keep invoking the source-size check as part of the normal contributor verification flow.
- If a temporary exception is necessary, record it in the allowlist with a short reviewable reason instead of silently accepting oversized files.
