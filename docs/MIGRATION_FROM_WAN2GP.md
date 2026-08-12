# Migration from Wan2GP prototype

The first AI Creator Studio implementation was developed under `ai_creator_studio/` in the Wan2GP repository. This repository is the clean product home.

## Keep

- Studio UI concepts
- Project/asset model
- Timeline model and editing behavior
- Preview behavior
- FFmpeg renderer integration
- Hardware/runtime policy concepts
- WanGP adapter concepts

## Do not copy as a second engine

- WanGP model code
- WanGP scheduler/runtime internals
- Deepy implementation
- duplicated model loaders

## Migration rule

Move application code into this repository and replace direct repository-relative assumptions with an explicit WanGP engine adapter. The application must be able to point at an installed/local WanGP runtime in development and a managed/bundled runtime in packaged distributions.

## Current source of truth

During migration, the prototype in `Zxcvb2521/Wan2GP` remains a reference snapshot. New product work should be committed here unless it is a deliberate upstream WanGP fix.
