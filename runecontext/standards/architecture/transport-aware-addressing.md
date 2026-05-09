---
schema_version: 1
id: architecture/transport-aware-addressing
title: Transport-Aware Addressing
status: active
tags:
  - architecture
  - addressing
  - host
  - device
  - workflows
---

# Transport-Aware Addressing

## Intent

Keep shared host, device, persistence, and workflow layers portable across local filesystem, scoped device paths, remote endpoints, and future transport-specific integrations.

## Requirements

- Shared domain and workflow layers must not assume every actionable location can be represented as a plain local filesystem path.
- Host and device contracts should expose addresses that preserve transport semantics such as local paths, transport-relative paths, remote endpoints, or logical identifiers.
- New integrations such as Android SAF, ADB, SSH, or similar transports should plug into existing addressing and capability contracts rather than forcing shared workflow rewrites.
- Persistence and logging surfaces should store address information in a way that preserves the transport meaning needed for review, replay, and diagnostics.
- Path-containment and write-safety checks must remain explicit about which guarantees apply to local filesystem paths versus other address kinds.

## Rationale

KOReader Companion is Kobo-first, but the roadmap already includes future ADB, SSH, Android-host, and other non-local-path workflows. If the shared core collapses all locations into plain local paths, later platform work will require architectural rework instead of additive target and host implementations.

## Implementation Notes

- Prefer transport-aware address/domain types over passing raw `PathBuf` values through shared interfaces.
- Device- or host-specific adapters may translate transport-aware addresses into concrete platform operations at the edges.
- Safety behavior should stay honest about what is verified for each address kind rather than implying local-filesystem guarantees where they do not exist.
