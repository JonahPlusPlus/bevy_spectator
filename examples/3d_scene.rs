//! A simple scene (based off Bevy's 3d_scene example).

use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .insert_resource(SpectatorSettings {
            base_speed: 5.0,
            alt_speed: 15.0,
            sensitivity: 0.0015,
            ..default()
        })
        .add_plugins((DefaultPlugins, SpectatorPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3)))
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        Spectator,
    ));
}
