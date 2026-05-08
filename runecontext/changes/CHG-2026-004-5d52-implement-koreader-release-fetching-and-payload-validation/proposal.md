## Summary
Implement KOReader release fetching and payload validation

## Problem
Fetch KOReader and launcher payloads from trusted release sources, select the correct Kobo artifact, validate downloaded files, and prepare deterministic install inputs.

## Proposed Change
Implement the payload/release module for the Kobo MVP. It should fetch KOReader release metadata from trusted sources, select the Kobo asset, validate checksums or release digests, cache release metadata, stage downloads, and validate extracted payload shape before any device write.

The initial KOReader Kobo matcher should target `koreader-kobo-v<version>.zip`. GitHub release asset digests should be preferred when present; fallback checksum parsing must be deterministic and fail closed when unavailable or ambiguous.

## Why Now
Install workflows must not select or write unvalidated payloads. This change separates release lookup and staging from device writes so CHG-005 can consume deterministic, inspectable payload decisions.

## Assumptions
- CHG-002 provides the payload, persistence, and safety boundaries.
- CHG-003 provides detected target information needed to choose Kobo payloads.
- Release metadata caching uses SQLite through the Rust core persistence layer.

## Out of Scope
- Writing staged payloads to devices.
- Installing KOReader or launcher integration.
- Selecting non-Kobo KOReader artifacts for post-MVP devices.

## Impact
This change makes release selection visible in dry-run plans and reduces the risk of installing the wrong KOReader payload.
