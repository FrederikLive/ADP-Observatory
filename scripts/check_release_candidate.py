#!/usr/bin/env python3
"""Fail closed when an experimental release candidate is not source/version-pinned.

This is a read-only build gate, not an authorization to publish a release.
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path


def validate(
    root: Path, expected_version: str, expected_sha: str, actual_sha: str, ref: str
) -> list[str]:
    problems: list[str] = []
    semver = r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-rc\.[1-9][0-9]*)?"
    if not re.fullmatch(semver, expected_version):
        problems.append("version must be a simple semver version (or -rc.N)")
    if not re.fullmatch(r"[0-9a-fA-F]{40}", expected_sha):
        problems.append("expected-sha must be a full, hexadecimal 40-character commit ID")
    if expected_sha.lower() != actual_sha.lower():
        problems.append("expected source SHA does not match the checkout")
    if ref != "refs/heads/main":
        problems.append("release candidate must be built from refs/heads/main")
    try:
        version = (root / "VERSION").read_text(encoding="utf-8").strip()
        if version != expected_version:
            problems.append("VERSION does not match the requested version")
    except OSError as error:
        problems.append(f"cannot read VERSION: {error}")
    try:
        import tomllib

        cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
        if cargo.get("package", {}).get("version") != expected_version:
            problems.append("Cargo.toml package version differs from release candidate")
        if cargo.get("package", {}).get("name") != "adp-observatory":
            problems.append("Cargo package name is not adp-observatory")
    except (OSError, ValueError, ImportError) as error:
        problems.append(f"cannot validate Cargo.toml: {error}")
    try:
        source = (root / "src/main.rs").read_text(encoding="utf-8")
        if f'schema_version: "{expected_version}"' not in source:
            problems.append("JSON report schema version differs from release candidate")
    except OSError as error:
        problems.append(f"cannot read scanner report version: {error}")
    return problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--version", required=True)
    parser.add_argument("--expected-sha", required=True)
    parser.add_argument("--ref", required=True)
    args = parser.parse_args()
    actual_sha = subprocess.check_output(
        ["git", "-C", str(args.root), "rev-parse", "HEAD"], text=True
    ).strip()
    issues = validate(args.root, args.version, args.expected_sha, actual_sha, args.ref)
    if issues:
        for issue in issues:
            print(f"RELEASE CANDIDATE REJECTED: {issue}", file=sys.stderr)
        return 1
    print(f"Verified candidate: v{args.version} @ {actual_sha}; no publication authorized")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
