# Project system

The project model is intentionally engine-independent.

A project contains:

- assets generated or imported by the user;
- timeline tracks and items;
- stable metadata and timestamps.

Generation engines return normalized results. The project layer turns those results into durable Studio assets.

The current store provides a browser-safe persistence layer for the prototype. Desktop persistence will move to the Tauri/backend filesystem layer without changing the project document schema.
