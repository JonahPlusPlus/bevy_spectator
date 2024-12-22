# Bevy Spectator

[![crates.io](https://img.shields.io/crates/v/bevy_spectator)](https://crates.io/crates/bevy_spectator)
[![crates.io](https://img.shields.io/crates/d/bevy_spectator)](https://crates.io/crates/bevy_spectator)
[![docs.rs](https://docs.rs/bevy_spectator/badge.svg)](https://docs.rs/bevy_spectator)

A spectator camera plugin for the [Bevy game engine](https://bevyengine.org/).

## Controls

| Action            | Key           |
|-------------------|---------------|
| Forward           | `W`           |
| Left              | `A`           |
| Backward          | `S`           |
| Right             | `D`           |
| Up                | `Space`       |
| Down              | `ControlLeft` |
| Alternative Speed | `ShiftLeft`   |
| Release Cursor    | `Escape`      |

Movement is constrained to the appropriate axes. (`WASD` to X & Z axes, `Space` & `ShiftLeft` to the Y axis)

When in orthographic mode, only `WASD` is used.

## Basic example

```rust,no_run
use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SpectatorPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(), Spectator
    ));
}
```

## Features

### `init`

Handles automatically setting `active_spectator` when there is exactly one camera with the `Spectator` component present.

Enabled by default.

### `bevy_egui`

Handles selectively disabling spectator mode entry when [bevy_egui](https://docs.rs/bevy_egui/latest/bevy_egui/) wants focus.

## Bevy compatibility

| bevy | bevy_spectator |
|------|----------------|
| 0.15 | 0.7            |
| 0.14 | 0.6            |
| 0.13 | 0.5            |
| 0.12 | 0.4            |
| 0.11 | 0.3            |
| 0.10 | 0.2            |
| 0.9  | 0.1            |
