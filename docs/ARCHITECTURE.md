# Architecture

## Boundary

```text
+-----------------------------+
|       AI Creator Studio     |
|                             |
| UI / Projects / Timeline    |
| Preview / Renderer          |
| Hardware / Runtime Policy   |
| Deepy Workspace             |
+--------------+--------------+
               |
        Stable adapter
               |
+--------------v--------------+
|            WanGP            |
|                             |
| Image / Video / Audio / TTS |
| Models / VRAM / Runtime     |
| Native Deepy                |
+-----------------------------+
```

## Adapter responsibilities

The adapter is responsible for translating application-level requests into WanGP requests and normalizing WanGP results. It must not duplicate model loading, VRAM management, scheduling or Deepy reasoning.

## Hardware strategy

The application detects available hardware and selects a compatible runtime/model configuration. No single GPU is a hard requirement. NVIDIA is the first-class Windows path, while other configurations are supported when the underlying WanGP runtime supports them.

## Deepy strategy

Deepy remains owned by WanGP. Studio exposes a workspace and integration contract around the native Deepy functionality. Studio stores resulting assets and workflow metadata, but does not implement a competing agent.

## Media flow

```text
User request
    |
    v
Studio request model
    |
    v
WanGP adapter
    |
    v
WanGP job
    |
    +--> image/video/audio/voice
    |
    v
Normalized result
    |
    +--> Project Asset
    +--> Timeline Item
    +--> Preview
    +--> Renderer
```

## Story workflow

A later workflow can use Deepy to produce a multi-step plan and then materialize approved outputs into the Studio project. The application remains responsible for user review, project state and final rendering.
