#!/usr/bin/env python3

from __future__ import annotations

import sys
import os
from pathlib import Path

ROOT = Path(os.environ.get("KC_SOURCE_SIZE_ROOT", Path(__file__).resolve().parent.parent)).resolve()
MAX_LINES = 320
INCLUDED_SUFFIXES = {".rs", ".py", ".sh", ".nix"}
INCLUDED_NAMES = {"Justfile"}
SKIP_PARTS = {"target", ".git", ".direnv", "result"}
ALLOWLIST = {}


def should_check(path: Path) -> bool:
    if any(part in SKIP_PARTS for part in path.parts):
        return False
    return path.suffix in INCLUDED_SUFFIXES or path.name in INCLUDED_NAMES


def line_count(path: Path) -> int:
    with path.open("r", encoding="utf-8", errors="ignore") as handle:
        return sum(1 for _ in handle)


def main() -> int:
    if not (ROOT / "Cargo.toml").exists() or not (ROOT / "Justfile").exists():
        print(f"source file size check expected repo root at {ROOT}", file=sys.stderr)
        return 1

    failures: list[tuple[str, int, str | None]] = []

    for path in sorted(ROOT.rglob("*")):
        if not path.is_file() or not should_check(path):
            continue

        rel = path.relative_to(ROOT).as_posix()
        lines = line_count(path)
        if lines <= MAX_LINES:
            continue

        failures.append((rel, lines, ALLOWLIST.get(rel)))

    unallowlisted = [entry for entry in failures if entry[2] is None]
    if unallowlisted:
        print(f"source file size check failed (limit: {MAX_LINES} lines)", file=sys.stderr)
        for rel, lines, _ in unallowlisted:
            print(f"- {rel}: {lines} lines", file=sys.stderr)
        if any(entry[2] is not None for entry in failures):
            print("allowlisted oversized files:", file=sys.stderr)
            for rel, lines, reason in failures:
                if reason is not None:
                    print(f"- {rel}: {lines} lines ({reason})", file=sys.stderr)
        return 1

    print(f"source file size check passed (limit: {MAX_LINES} lines)")
    if failures:
        print("allowlisted oversized files:")
        for rel, lines, reason in failures:
            if reason is not None:
                print(f"- {rel}: {lines} lines ({reason})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
