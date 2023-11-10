//! A simple 2D scene using orthographic controls.

use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .insert_resource(SpectatorSettings {
            orthographic: true,
            ..default()
        })
        .add_plugins((DefaultPlugins, SpectatorPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Spectator));
    commands.spawn(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 1.0),
            ..default()
        },
        ..default()
    });
}
