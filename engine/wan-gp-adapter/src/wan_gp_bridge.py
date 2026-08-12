"""Small localhost bridge over WanGP's official in-process shared.api.

This is intentionally a thin adapter: generation, model definitions and schemas
remain owned by WanGP. The Studio only translates JSON requests into shared.api
calls and exposes job state to the desktop UI.
"""
from __future__ import annotations

import json
import os
import sys
import threading
import traceback
import uuid
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("WAN2GP_ROOT") or os.environ.get("WAN_GP_ROOT") or "Wan2GP").expanduser().resolve()
PORT = int(os.environ.get("AI_CREATOR_WAN_PORT", "18765"))

sys.path.insert(0, str(ROOT))

from shared.api import WanGPSession  # noqa: E402

SESSION = WanGPSession(root=ROOT, console_output=False)
JOBS: dict[str, dict[str, Any]] = {}
LOCK = threading.RLock()


def json_safe(value: Any) -> Any:
    if value is None or isinstance(value, (str, int, float, bool)):
        return value
    if isinstance(value, Path):
        return str(value)
    if isinstance(value, dict):
        return {str(k): json_safe(v) for k, v in value.items()}
    if isinstance(value, (list, tuple)):
        return [json_safe(v) for v in value]
    return str(value)


def model_catalog() -> list[dict[str, Any]]:
    return json_safe(SESSION.list_model_metadata(include_availability=True))


def run_job(job_id: str, settings: dict[str, Any]) -> None:
    with LOCK:
        JOBS[job_id]["status"] = "running"
        JOBS[job_id]["phase"] = "WanGP generation"
        JOBS[job_id]["progress"] = 0
    try:
        job = SESSION.submit_task(settings)
        with LOCK:
            JOBS[job_id]["wan_job"] = job
        result = job.result()
        payload = {
            "success": result.success,
            "files": json_safe(result.generated_files),
            "errors": [json_safe(error.message) for error in result.errors],
            "total_tasks": result.total_tasks,
            "successful_tasks": result.successful_tasks,
            "failed_tasks": result.failed_tasks,
            "artifacts": json_safe(result.artifacts),
        }
        with LOCK:
            JOBS[job_id].update(status="completed" if result.success else "failed", progress=100 if result.success else 0, result=payload)
    except Exception as exc:
        with LOCK:
            JOBS[job_id].update(status="failed", error=str(exc), traceback=traceback.format_exc())


class Handler(BaseHTTPRequestHandler):
    server_version = "AI-Creator-WanGP-Bridge/1.0"

    def log_message(self, *_args: Any) -> None:
        return

    def send_json(self, status: int, payload: Any) -> None:
        body = json.dumps(json_safe(payload), ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def read_json(self) -> dict[str, Any]:
        length = int(self.headers.get("Content-Length", "0"))
        raw = self.rfile.read(length) if length else b"{}"
        value = json.loads(raw.decode("utf-8"))
        return value if isinstance(value, dict) else {}

    def do_GET(self) -> None:  # noqa: N802
        try:
            if self.path == "/health":
                self.send_json(200, {"engine": "WanGP", "root": str(ROOT), "status": "ready"})
                return
            if self.path == "/models":
                self.send_json(200, {"models": model_catalog()})
                return
            if self.path.startswith("/models/"):
                model_type = self.path.split("/", 2)[2]
                schema = SESSION.get_model_schema(model_type)
                if schema is None:
                    self.send_json(404, {"error": f"Unknown model_type: {model_type}"})
                else:
                    self.send_json(200, schema)
                return
            if self.path.startswith("/jobs/"):
                job_id = self.path.split("/", 2)[2]
                with LOCK:
                    job = JOBS.get(job_id)
                    if not job:
                        self.send_json(404, {"error": "Job not found"})
                        return
                    payload = {k: v for k, v in job.items() if k != "wan_job"}
                self.send_json(200, payload)
                return
            self.send_json(404, {"error": "Not found"})
        except Exception as exc:
            self.send_json(500, {"error": str(exc)})

    def do_POST(self) -> None:  # noqa: N802
        try:
            if self.path in ("/generate", "/generate/video", "/generate/image"):
                body = self.read_json()
                settings = dict(body.get("settings") or {})
                prompt = str(body.get("prompt") or "")
                if prompt:
                    settings["prompt"] = prompt
                if body.get("negative_prompt"):
                    settings["negative_prompt"] = body["negative_prompt"]
                if body.get("model"):
                    settings["model_type"] = body["model"]
                if self.path.endswith("/video"):
                    settings.setdefault("model_type", body.get("model"))
                job_id = uuid.uuid4().hex
                with LOCK:
                    JOBS[job_id] = {"id": job_id, "status": "queued", "progress": 0, "phase": "Queued"}
                threading.Thread(target=run_job, args=(job_id, settings), daemon=True).start()
                self.send_json(202, {"job_id": job_id})
                return
            if self.path.startswith("/jobs/") and self.path.endswith("/cancel"):
                job_id = self.path.split("/")[2]
                with LOCK:
                    record = JOBS.get(job_id)
                    wan_job = record.get("wan_job") if record else None
                if wan_job is None:
                    self.send_json(404, {"error": "Job not found"})
                    return
                wan_job.cancel()
                with LOCK:
                    record["status"] = "cancelled"
                self.send_json(200, {"ok": True})
                return
            self.send_json(404, {"error": "Not found"})
        except Exception as exc:
            self.send_json(500, {"error": str(exc)})


def main() -> None:
    SESSION.ensure_ready()
    server = ThreadingHTTPServer(("127.0.0.1", PORT), Handler)
    print(f"AI Creator Studio WanGP bridge listening on 127.0.0.1:{PORT}", flush=True)
    try:
        server.serve_forever()
    finally:
        server.server_close()
        SESSION.close()


if __name__ == "__main__":
    main()
