//! A simple 2D scene using orthographic controls.

use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .insert_resource(SpectatorSettings {
            base_speed: 100.0,
            alt_speed: 350.0,
            orthographic: true,
            ..default()
        })
        .add_plugins((DefaultPlugins, SpectatorPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Sprite
    commands.spawn((
        Sprite::default(),
        Transform::default().with_scale(Vec3::new(10.0, 10.0, 1.0)),
    ));

    // Camera
    commands.spawn((
        Camera2d::default(),
        Spectator,
    ));
}
