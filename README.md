# bevy_spectator
[![bevy](https://img.shields.io/badge/Bevy-0.9-blue)](https://crates.io/crates/bevy/0.9.0)
[![Crates.io](https://img.shields.io/crates/v/bevy_spectator)](https://crates.io/crates/bevy_spectator)
[![Crates.io](https://img.shields.io/crates/d/bevy_spectator)](https://crates.io/crates/bevy_spectator)
[![docs.rs](https://img.shields.io/docsrs/bevy_spectator)](https://docs.rs/bevy_spectator/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/JonahPlusPlus/bevy_spectator#license)

A spectator camera plugin for the [Bevy game engine](https://bevyengine.org/).

## Controls

|Action|Key|
|-|-|
|Forward|`W`|
|Left|`A`|
|Backward|`S`|
|Right|`D`|
|Up|`Space`|
|Down|`LControl`|
|Alt. Speed|`LShift`|
|Release Cursor|`Escape`|

Movement is constrained to the appropriate axes. (`WASD` to X & Z axes, `Space` & `LShift` to the Y axis)

## `basic` Example
```
use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpectatorPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(), Spectator
    ));
}
```

## License
bevy_spectator is dual-licensed under MIT/Apache-2.0. Feel free to use it under either.
