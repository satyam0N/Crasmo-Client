# Crasmo Client

Crasmo Client is an independent Minecraft Java launcher project using Tauri 2 and React. It is not affiliated with Mojang or Microsoft and contains no official Minecraft artwork or textures.

## Run locally

Requirements: Node.js 20+, Rust stable, and the platform prerequisites for Tauri 2 (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)).

```sh
npm install
npm run tauri dev
```

For browser-only UI development use `npm run dev`. Build the frontend with `npm run build`; build a native bundle with `npm run tauri build` on Windows, macOS, or Linux. Native installers must be produced on their respective platforms.

## Current implementation

The React shell includes Home, Instances, Modrinth search, Skins, and Settings pages, custom window controls, a command palette, instance selection, and a persistent glass play bar. The Tauri backend now persists instance metadata and validated settings in the application data directory, creates isolated instance directories, and fetches/caches Mojang's version manifest with offline cache fallback. The frontend hydrates and persists desktop instance creations/deletions through typed invoke wrappers. Rust unit tests cover instance name validation and settings validation.

To enable Microsoft sign-in in a future implementation, register an application in Azure with the appropriate public-client/device-code flow. Do not store client secrets or access/refresh tokens in source control.

## Not yet implemented

This is not yet a playable launcher: Microsoft/Xbox/Minecraft authentication and OS keychain storage, SHA-1 verified/resumable game downloads and progress cancellation, complete libraries/assets/natives installation, JVM argument construction and process supervision, Fabric/Forge/Quilt installation, dependency-aware mod downloads, and account-backed skin upload/3D preview remain unfinished. The frontend marks those operations as unavailable rather than faking success. Cross-platform native bundles and runtime behavior still need to be verified on Windows, macOS, and Linux.

Run `cargo test --manifest-path src-tauri/Cargo.toml` to execute backend unit tests and `npm run build` for the frontend production build.
