# Timeline

The timeline is a non-destructive arrangement of project assets.

Generated/imported media remains an asset; timeline items reference the asset by ID. This makes it possible to reuse the same image, video, voice or music clip without duplicating the underlying file.

Current automatic placement rules:

- image/video → Video track
- voice → Voice track
- audio → Music track

New generated assets can therefore follow the complete path:

`WanGP job → normalized result → project asset → timeline item`.

The next implementation layer is the visual timeline editor and media preview. The model already supports moving and removing items without changing the source asset.
