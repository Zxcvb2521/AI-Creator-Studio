"""Small subprocess adapter around WanGP's documented in-process Python API.

The Studio never reimplements WanGP's model registry. It asks WanGP itself for
model definitions/metadata and serializes only the data needed by the desktop UI.
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path


def load_session(root: Path):
    sys.path.insert(0, str(root))
    from shared.api import init  # type: ignore
    return init(root=root, console_output=False, console_isatty=False)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True)
    parser.add_argument("command", choices=["models"])
    args = parser.parse_args()
    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        raise RuntimeError(f"WanGP root does not exist: {root}")

    session = load_session(root)
    if args.command == "models":
        records = session.list_model_metadata(include_availability=True)
        print(json.dumps(records, ensure_ascii=False, default=str))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"error": str(exc)}, ensure_ascii=False), file=sys.stderr)
        raise SystemExit(1)
