from __future__ import annotations

import argparse
import tarfile
import zipfile
from pathlib import Path, PurePosixPath


BANNED_PARTS = {
    "__pycache__",
    ".pytest_cache",
    ".ruff_cache",
    ".venv",
}
BANNED_SUFFIXES = {".pyc", ".pyo"}
REQUIRED_WHEEL_FILES = {
    PurePosixPath("techr/__init__.py"),
    PurePosixPath("techr/types.py"),
}
REQUIRED_SDIST_FILES = {
    PurePosixPath("pyproject.toml"),
    PurePosixPath("Cargo.toml"),
    PurePosixPath("README.md"),
    PurePosixPath("techr/__init__.py"),
}


def load_members(path: Path) -> list[PurePosixPath]:
    if path.suffix == ".whl":
        with zipfile.ZipFile(path) as archive:
            return [PurePosixPath(name) for name in archive.namelist()]
    if path.suffixes[-2:] == [".tar", ".gz"]:
        with tarfile.open(path, "r:gz") as archive:
            return [PurePosixPath(name) for name in archive.getnames()]
    raise ValueError(f"Unsupported artifact format: {path}")


def strip_root(paths: list[PurePosixPath]) -> list[PurePosixPath]:
    stripped: list[PurePosixPath] = []
    for path in paths:
        if len(path.parts) <= 1:
            continue
        stripped.append(PurePosixPath(*path.parts[1:]))
    return stripped


def has_suffix_path(path: PurePosixPath, suffix: tuple[str, ...]) -> bool:
    return len(path.parts) >= len(suffix) and path.parts[-len(suffix) :] == suffix


def assert_no_banned_entries(path: Path, members: list[PurePosixPath]) -> None:
    for member in members:
        if any(part in BANNED_PARTS for part in member.parts):
            raise ValueError(f"{path.name} contains banned path: {member.as_posix()}")
        if member.suffix in BANNED_SUFFIXES:
            raise ValueError(f"{path.name} contains banned file: {member.as_posix()}")


def validate_wheel(path: Path, members: list[PurePosixPath]) -> None:
    required_missing = sorted(
        file.as_posix() for file in REQUIRED_WHEEL_FILES if file not in members
    )
    if required_missing:
        raise ValueError(f"{path.name} is missing files: {', '.join(required_missing)}")

    native_extensions = [
        member
        for member in members
        if member.parent == PurePosixPath("techr")
        and member.name.startswith("_techr.")
        and member.suffix in {".so", ".pyd"}
    ]
    if len(native_extensions) != 1:
        found = ", ".join(member.as_posix() for member in native_extensions) or "none"
        raise ValueError(
            f"{path.name} must contain exactly one native extension, found {found}"
        )


def validate_sdist(path: Path, members: list[PurePosixPath]) -> None:
    normalized = strip_root(members)
    missing_files = sorted(
        file.as_posix() for file in REQUIRED_SDIST_FILES if file not in normalized
    )
    if missing_files:
        raise ValueError(f"{path.name} is missing files: {', '.join(missing_files)}")

    required_patterns = {
        "techr/*.py": any(
            member.suffix == ".py"
            and len(member.parts) >= 2
            and member.parts[-2] == "techr"
            for member in normalized
        ),
        "src/*.rs": any(
            member.suffix == ".rs"
            and has_suffix_path(member.parent, ("src",))
            for member in normalized
        ),
    }
    missing_patterns = sorted(
        pattern for pattern, matched in required_patterns.items() if not matched
    )
    if missing_patterns:
        raise ValueError(
            f"{path.name} is missing required source patterns: {', '.join(missing_patterns)}"
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifact_dir", type=Path)
    args = parser.parse_args()

    artifact_dir = args.artifact_dir.resolve()
    artifacts = sorted(artifact_dir.glob("*.whl")) + sorted(artifact_dir.glob("*.tar.gz"))
    if not artifacts:
        raise SystemExit(f"No artifacts found in {artifact_dir}")

    for artifact in artifacts:
        members = load_members(artifact)
        assert_no_banned_entries(artifact, members)
        if artifact.suffix == ".whl":
            validate_wheel(artifact, members)
        else:
            validate_sdist(artifact, members)
        print(f"checked {artifact.name}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
