# AI Creator Studio

A standalone desktop application for AI-assisted image, video, voice, music, story and timeline creation.

## Architecture

AI Creator Studio is the application layer. WanGP remains the generation engine and is consumed through a stable adapter/runtime boundary.

```text
AI Creator Studio
        |
        v
   WanGP Adapter
        |
        v
      WanGP
   /    |     \
 Image Video  Audio/TTS
        |
      Deepy
```

Deepy is **not** reimplemented here. The application integrates with the Deepy capability already provided by WanGP.

## Goals

- Windows-first desktop application
- Support different NVIDIA/AMD hardware and different VRAM sizes
- Automatic hardware/runtime policy
- Image and video generation through WanGP
- Native WanGP audio/TTS capabilities
- Project assets and non-destructive timeline
- Preview and FFmpeg rendering
- Deepy workspace for multi-step generation workflows
- Story-to-video workflow without locking the application to one GPU

## Development status

The project is being extracted from the prototype developed inside the Wan2GP repository. The original Wan2GP repository remains the engine/reference implementation; this repository is the product application.

## Repository layout

```text
app/          desktop application
engine/       WanGP adapter and integration contract
projects/     project persistence
media/        media asset handling
timeline/     timeline model and editing
renderer/     preview/export
hardware/     hardware detection and runtime policy
deepy/        Deepy integration layer (no reimplementation)
installer/    platform packaging
 docs/        architecture and development documentation
```

## License

Информацию о лицензировании ядра см. в исходном проекте WanGP. Лицензирование приложения будет определено отдельно.
