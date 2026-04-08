from __future__ import annotations

import re
import sys
from pathlib import Path


TAG_PREFIX = "polars-v"


def read_section_version(path: Path, section: str) -> str:
    text = path.read_text()
    section_pattern = re.compile(
        rf"(?ms)^\[{re.escape(section)}\]\s*(.*?)(?=^\[|\Z)"
    )
    section_match = section_pattern.search(text)
    if section_match is None:
        raise ValueError(f"Could not find [{section}] in {path}")

    version_pattern = re.compile(r'(?m)^version\s*=\s*"([^"]+)"\s*$')
    version_match = version_pattern.search(section_match.group(1))
    if version_match is None:
        raise ValueError(f"Could not find version in [{section}] of {path}")

    return version_match.group(1)


def require_project_dynamic_version(path: Path) -> None:
    text = path.read_text()
    section_pattern = re.compile(r"(?ms)^\[project\]\s*(.*?)(?=^\[|\Z)")
    section_match = section_pattern.search(text)
    if section_match is None:
        raise ValueError(f"Could not find [project] in {path}")

    dynamic_pattern = re.compile(r'(?m)^dynamic\s*=\s*\[(.*?)\]\s*$')
    dynamic_match = dynamic_pattern.search(section_match.group(1))
    if dynamic_match is None or '"version"' not in dynamic_match.group(1):
        raise ValueError(
            f"{path} must declare dynamic = [\"version\"] in [project]"
        )


def normalize_tag(tag: str) -> str:
    if tag.startswith("refs/tags/"):
        tag = tag.removeprefix("refs/tags/")
    return tag


def main() -> int:
    polars_dir = Path(__file__).resolve().parents[1]
    pyproject_path = polars_dir / "pyproject.toml"
    require_project_dynamic_version(pyproject_path)
    cargo_version = read_section_version(polars_dir / "Cargo.toml", "package")

    if len(sys.argv) > 2:
        print("Usage: check_versions.py [polars-vX.Y.Z]", file=sys.stderr)
        return 1

    if len(sys.argv) == 2:
        tag = normalize_tag(sys.argv[1])
        if not tag.startswith(TAG_PREFIX):
            print(
                f"Expected a tag starting with {TAG_PREFIX!r}, got {tag!r}.",
                file=sys.stderr,
            )
            return 1
        tag_version = tag.removeprefix(TAG_PREFIX)
        if tag_version != cargo_version:
            print(
                "Tag version mismatch:",
                f"tag={tag_version}",
                f"package={cargo_version}",
                file=sys.stderr,
            )
            return 1

    print(f"polars-techr version: {cargo_version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
