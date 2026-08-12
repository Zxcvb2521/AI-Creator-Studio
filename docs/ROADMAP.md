# AI Creator Studio — Roadmap

## Product principle

AI Creator Studio is a standalone application. WanGP is the engine. We do not fork or reimplement WanGP/Deepy inside the product unless an explicit integration boundary requires it.

## Phase 0 — Engine contract

- Stable WanGP adapter
- Capability discovery
- Job submission/status/events
- Media result normalization
- Deepy capability detection and invocation boundary
- Version compatibility checks

## Phase 1 — Desktop shell

- Tauri desktop application
- Windows-first packaging
- Development and bundled runtime modes
- Hardware detection
- VRAM/RAM/CPU-aware runtime policy
- Graceful CPU/low-VRAM fallback where supported

## Phase 2 — Project system

- Projects
- Assets
- Metadata
- Import/export
- Persistent workspace

## Phase 3 — Generation workspace

- Image generation
- Video generation
- Audio/music generation
- Voice/TTS generation
- Unified job monitor
- Result-to-asset flow

## Phase 4 — Timeline and rendering

- Video/audio tracks
- Drag/drop
- Trim/resize
- Ordering
- Preview
- FFmpeg export
- Real media duration detection

## Phase 5 — Deepy workspace

- Use the Deepy already shipped by WanGP
- Prompt/chat workspace
- Capability-aware request routing
- Show generated artifacts in the Studio project
- Convert multi-step Deepy results into project assets/timeline items

## Phase 6 — Story-to-video

- Idea → script
- Script → scenes
- Scene prompts
- Character/style consistency metadata
- Batch image/video generation
- Voice/music assignment
- Automatic timeline assembly
- Human review before final render

## Phase 7 — Portability and distribution

- Windows installer
- Portable mode
- NVIDIA variants
- AMD variants where upstream supports them
- CPU/low-memory degradation modes
- Runtime diagnostics
- Model management
- Reproducible environment reports

## Phase 8 — Advanced workflows

- Templates
- Presets
- Batch jobs
- Recovery/resume
- Project versioning
- Optional external LLM providers
- Optional external TTS/music providers

## Explicit non-goals

- Do not create a second Deepy implementation.
- Do not hard-code RTX 5060 Ti requirements.
- Do not copy the complete WanGP repository into the application repository.
- Do not make Ollama/KoboldCpp mandatory.
- Do not replace WanGP's native inference/runtime systems when an adapter is sufficient.
