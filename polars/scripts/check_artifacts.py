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
    PurePosixPath("polars_techr/__init__.py"),
    PurePosixPath("polars_techr/types.py"),
}
REQUIRED_SDIST_SUFFIXES = {
    "pyproject.toml",
    "Cargo.toml",
    "README.md",
    "polars_techr/__init__.py",
    "polars_techr/types.py",
    "src/lib.rs",
    "src/expressions.rs",
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
        if member.parent == PurePosixPath("polars_techr")
        and member.name.startswith("_polars_techr.")
        and member.suffix in {".so", ".pyd"}
    ]
    if len(native_extensions) != 1:
        found = ", ".join(member.as_posix() for member in native_extensions) or "none"
        raise ValueError(
            f"{path.name} must contain exactly one native extension, found {found}"
        )


def validate_sdist(path: Path, members: list[PurePosixPath]) -> None:
    normalized = [member.as_posix() for member in strip_root(members)]
    missing = sorted(
        name for name in REQUIRED_SDIST_SUFFIXES if not any(
            member == name or member.endswith(f"/{name}") for member in normalized
        )
    )
    if missing:
        raise ValueError(f"{path.name} is missing files: {', '.join(missing)}")


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
