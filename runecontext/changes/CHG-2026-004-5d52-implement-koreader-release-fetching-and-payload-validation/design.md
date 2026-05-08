# Design

## Overview
Implement a Rust payload module that turns trusted release metadata into validated staged install inputs. Device workflows should receive a typed payload decision, not raw GitHub response data or unchecked archive paths.

## Release Metadata

- Use the KOReader GitHub Releases API or equivalent trusted endpoint for stable releases.
- Cache release metadata in SQLite to reduce rate-limit exposure and support repeat dry-runs.
- Use conditional requests when practical.
- Make rate-limit, network, and cache-staleness state visible in diagnostic output.
- Do not require a user GitHub token for MVP.

## Kobo Artifact Selection

- Match Kobo assets by `koreader-kobo-v<version>.zip` unless upstream changes require an updated rule.
- Reject releases with zero or multiple matching Kobo assets unless the user explicitly selects a local artifact or a later rule resolves the ambiguity.
- Treat release candidates, nightlies, and prereleases as out of scope unless explicitly enabled by a later change.
- Store the selected version, asset name, source URL, digest/checksum source, and retrieval timestamp.

## Validation And Staging

- Prefer GitHub release asset `sha256:` digests when available.
- Fall back to published release checksum metadata only when the parser is deterministic and tested.
- Validate archive structure before it is eligible for installation. At minimum, staged content must contain `koreader/` and the expected Kobo launch script path.
- Extract into a local staging directory first, never directly onto the device.
- Preserve enough metadata for CHG-005 to display exactly what will be copied.

## Local Artifact Support

Allow an advanced local-artifact path only if it passes the same Kobo archive validation and checksum handling rules. If no checksum is available, the dry-run should clearly mark the reduced trust level and require explicit confirmation before any later write workflow.

## Research Sources

- KOReader releases: `https://github.com/koreader/koreader/releases`.
- GitHub Releases API asset digest field.
- KOReader Kobo installation wiki: `https://github.com/koreader/koreader/wiki/Installation-on-Kobo-devices`.
