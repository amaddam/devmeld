"""Check or materialize the two project-owned Codex command overrides."""

import argparse
from pathlib import Path
import sys


COMMANDS = ("tasks", "implement")


def sync(root: Path, *, write: bool = False) -> bool:
    root = root.resolve()
    pending = []
    for name in COMMANDS:
        source = root / ".specify/templates/overrides" / f"{name}.md"
        target = root / ".agents/skills" / f"speckit-{name}" / "SKILL.md"
        for path in (source, target):
            resolved = path.resolve()
            if not resolved.is_relative_to(root):
                raise ValueError(f"Path escapes project: {path}")
            if resolved != path:
                raise ValueError(f"Refusing redirected project file: {path}")
        expected = source.read_text(encoding="utf-8")
        actual = target.read_text(encoding="utf-8") if target.exists() else None
        if actual != expected:
            pending.append((target, expected))

    # Read both sources before writing either output, so a missing source cannot
    # leave a partially synchronized pair. This copies text with normalized
    # newlines, not general Spec Kit rendering; sources use skill frontmatter.
    for target, expected in pending:
        if write:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(expected, encoding="utf-8", newline="\n")
        print(f"{'Updated' if write else 'Out of sync'}: {target.relative_to(root)}")
    if not pending:
        print("Both project-owned skills match their command overrides.")
    return write or not pending


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="Read-only check (default)")
    mode.add_argument("--write", action="store_true", help="Replace only the two generated skill copies")
    args = parser.parse_args()
    try:
        return 0 if sync(Path(__file__).resolve().parents[2], write=args.write) else 1
    except (OSError, ValueError) as error:
        print(f"Skill synchronization failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
