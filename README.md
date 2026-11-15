# Crasmo Client

An independent Minecraft Java launcher UI and Tauri 2 desktop application scaffold. The current commit delivers a working React launcher shell with navigable Home, Instances, Mods & Packs (live Modrinth search), Skins, and Settings screens. Instance creation/filtering, selection, the command palette, custom window controls, and local UI settings are interactive.

> Crasmo Client is not affiliated with Mojang or Microsoft. This project uses no official game artwork or textures.

## Requirements
- Node.js 20+ and npm
- Rust stable with the platform build prerequisites for Tauri 2 (see https://v2.tauri.app/start/prerequisites/)
- For Microsoft authentication, register an application in the Azure portal and configure a public-client/device-code flow. Never commit client secrets or access/refresh tokens.

## Run
```sh
npm install
npm run tauri dev
```
For browser-only UI development: `npm run dev`.
Build the frontend with `npm run build`; build platform installer/bundle targets with `npm run tauri build` on Windows, macOS, or Linux. Native bundles need to be built on their respective platforms.

## Implementation status
The current deliverable is the launcher foundation, not a fully playable game installer. Rust modules and typed error boundaries are laid out by domain; the exposed backend currently includes health/settings/instance-list scaffolding. Authentication and keychain storage, Mojang version/install manifest, verified/resumable downloads, real instance persistence, JVM argument generation/process management, loader installation, Modrinth dependency-aware installs, profile skin API/3D preview, and cross-platform installer validation remain to be implemented. The frontend intentionally labels launch/upload actions as unavailable rather than claiming success. No credentials are embedded.

## Next phases
1. Persist instance/settings models and typed command errors in app data.
2. Implement manifest + artifact installation with SHA-1 validation, progress events, cancellation, and tests.
3. Add Microsoft device-code/Xbox/Minecraft auth using OS credential storage.
4. Construct/test JVM launch arguments and supervise game processes.
5. Integrate Fabric/Forge/Quilt and Modrinth dependency resolution.
6. Add account-backed skin upload/profile updates, real 3D preview, accessibility/platform QA.
