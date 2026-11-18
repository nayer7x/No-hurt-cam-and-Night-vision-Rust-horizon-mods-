# Minecraft Horizon Rust ports

Two experimental native-Horizon mod projects based on the uploaded Java mods:

- `fullbright/`: Fullbright Multiplayer logic port.
- `no_hurtcam/`: No Hurt Cam camera-rotation logic port.

## Status and limitations

Both projects have a `horizon-mod.toml`, `Cargo.toml`, SDK dependency and a native mod entry point. **Neither is confirmed to work in-game.** The Rust algorithms are not connected to client/game hooks: Fullbright does not apply the night-vision effect and No Hurt Cam does not intercept camera shake yet. Adding a native entry point is not sufficient to reproduce mod behavior.

Build, run-time and launcher validation have not been performed. The manifest pins were copied from the official [Horizon machine sample](https://github.com/arismoko/horizon-machine-sample), and may be incompatible with your installed launcher/game version. Match the game's release pins and SDK before building.

## Install or check

The launcher may discover the two nested mods when importing the GitHub repository. If it does not, supply each mod's own source location or package them separately following the official [native-mod documentation](https://github.com/arismoko/horizon/blob/main/docs/modding/src/how-to/native-mods/07-local-developer-builds.md).

Do not interpret launcher discovery as proof of compilation or functionality. These projects still require implementing and testing real game/client hooks.

## Layout

```text
fullbright/
  horizon-mod.toml
  Cargo.toml
  src/lib.rs
no_hurtcam/
  horizon-mod.toml
  Cargo.toml
  src/lib.rs
```
