"""Small subprocess adapter around WanGP's documented in-process Python API.

The Studio delegates model discovery, defaults, schema, and generation to the
installed WanGP runtime instead of reimplementing its model registry or engine.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


def load_session(root: Path, output_dir: Path | None = None):
    sys.path.insert(0, str(root))
    from shared.api import init  # type: ignore
    return init(
        root=root,
        output_dir=output_dir,
        console_output=False,
        console_isatty=False,
    )


def read_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"Settings file must contain a JSON object: {path}")
    return value


def json_out(value: Any) -> None:
    print(json.dumps(value, ensure_ascii=False, default=str))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True)
    parser.add_argument("--output-dir")
    parser.add_argument("command", choices=["models", "schema", "generate"])
    parser.add_argument("--model", dest="model_type")
    parser.add_argument("--settings")
    args = parser.parse_args()

    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        raise RuntimeError(f"WanGP root does not exist: {root}")
    output_dir = Path(args.output_dir).expanduser().resolve() if args.output_dir else None
    session = load_session(root, output_dir)

    if args.command == "models":
        json_out(session.list_model_metadata(include_availability=True))
        return 0

    if not args.model_type:
        raise ValueError("--model is required for schema")

    if args.command == "schema":
        schema = session.get_model_schema(args.model_type)
        if schema is None:
            raise ValueError(f"Unknown model_type: {args.model_type}")
        json_out(schema)
        return 0

    if not args.settings:
        raise ValueError("--settings is required for generate")
    request = read_json(Path(args.settings).expanduser().resolve())
    model_type = str(request.get("model_type") or args.model_type or "").strip()
    if not model_type:
        raise ValueError("settings must contain model_type")

    # Start from Wan2GP's own defaults, then apply only the request overrides.
    settings = session.get_default_settings(model_type)
    settings.update(request)
    settings["model_type"] = model_type

    job = session.submit_task(settings)
    result = job.result()
    json_out({
        "success": result.success,
        "generated_files": result.generated_files,
        "errors": [
            {
                "message": error.message,
                "task_index": error.task_index,
                "task_id": error.task_id,
                "stage": error.stage,
                "cancelled": error.cancelled,
            }
            for error in result.errors
        ],
        "total_tasks": result.total_tasks,
        "successful_tasks": result.successful_tasks,
        "failed_tasks": result.failed_tasks,
        "artifacts": [
            {
                "path": artifact.path,
                "media_type": artifact.media_type,
                "client_id": artifact.client_id,
                "hdr": artifact.hdr,
                "fps": artifact.fps,
            }
            for artifact in result.artifacts
        ],
    })
    return 0 if result.success else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        json_out({"error": str(exc)})
        raise SystemExit(1)
