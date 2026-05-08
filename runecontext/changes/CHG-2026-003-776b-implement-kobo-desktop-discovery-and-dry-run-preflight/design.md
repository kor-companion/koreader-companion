# Design

## Overview
Implement Kobo discovery as a device target module plus desktop host discovery adapters. The workflow must be read-only and produce a dry-run plan that later install and backup workflows can reuse.

## Detection Rules

- Prefer host-provided removable volume metadata with filesystem label `KOBOeReader` when available.
- Confirm the selected root contains `.kobo/`.
- Classify install-capable Kobo roots only when `.kobo/Kobo/Kobo eReader.conf` is present.
- Treat `.kobo/` without the expected config path as ambiguous unless a later compatibility rule explicitly supports it.
- Support manual path selection, but apply the same sentinel checks as automatic discovery.
- Keep all detection reads path-contained under the selected root.

## Existing State Checks

The preflight should detect and report:

- Existing `.adds/koreader` and `.adds/koreader/koreader.sh`.
- Existing KFMon state under `.adds/kfmon` and config entries when present.
- Existing NickelMenu state under `.adds/nm` when present.
- Launcher icons such as `koreader.png` when present.
- Pending `.kobo/KoboRoot.tgz` files that may indicate an incomplete or pending launcher install.
- Writable status, available space, and read errors from the host adapter.

## Dry-Run Plan

The dry-run plan should show:

- Detected host adapter and selected root.
- Device classification and support level.
- Existing KOReader and launcher state.
- Planned reads for later workflows.
- Planned writes that would occur during installation, without performing them.
- Reasons the workflow is blocked, degraded, or safe to continue.

## Research Sources

- KOReader Kobo installation wiki: `https://github.com/koreader/koreader/wiki/Installation-on-Kobo-devices`.
- KFMon installer and config examples: `https://github.com/NiLuJe/kfmon`.
