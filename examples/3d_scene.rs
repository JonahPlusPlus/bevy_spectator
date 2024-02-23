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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 1.5, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Spectator,
    ));
}
