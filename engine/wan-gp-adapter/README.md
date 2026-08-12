# WanGP Adapter

This package is the only application-facing boundary for WanGP.

## Responsibilities

- Discover engine capabilities.
- Submit normalized generation jobs.
- Read job status.
- Cancel jobs.
- Normalize the engine boundary for the Studio.

## Deliberate boundary

The adapter does not load models, manage VRAM, implement schedulers, or implement Deepy. Those responsibilities stay in WanGP.

## Studio API contract

The current HTTP transport expects:

- `GET /api/studio/capabilities`
- `POST /api/studio/jobs`
- `GET /api/studio/jobs/:id`
- `POST /api/studio/jobs/:id/cancel`

These are the application integration endpoints. The WanGP-side Studio backend will expose them using the real WanGP runtime; the application must not mock or duplicate inference logic.

## Runtime modes

The adapter will support:

1. Existing local WanGP checkout/runtime (development).
2. User-selected installed WanGP runtime.
3. Managed/bundled runtime for packaged distributions.
