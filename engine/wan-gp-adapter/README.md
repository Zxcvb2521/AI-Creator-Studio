# WanGP Adapter

This package is the only application-facing boundary to the WanGP engine.

## Responsibilities

- Discover supported media capabilities.
- Submit generation requests.
- Track jobs.
- Cancel jobs.
- Normalize generated files and metadata.
- Expose whether native WanGP Deepy is available.

## Non-responsibilities

The adapter does **not** load models, manage VRAM, implement schedulers, or reimplement Deepy. Those remain WanGP responsibilities.

## Runtime modes

The adapter will support:

1. Existing local WanGP checkout/runtime (development).
2. User-selected installed WanGP runtime.
3. Managed/bundled runtime for packaged distributions.

The exact transport is deliberately isolated behind this interface so the Studio UI does not depend on WanGP's internal Python layout.
