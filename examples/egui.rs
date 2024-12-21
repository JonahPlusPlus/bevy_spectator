//! The same as 3d_scene but showing the `bevy_egui` integration.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_spectator::*;

fn main() {
    App::new()
        .insert_resource(SpectatorSettings {
            base_speed: 5.0,
            alt_speed: 15.0,
            sensitivity: 0.0015,
            ..default()
        })
        .add_plugins((DefaultPlugins, EguiPlugin, SpectatorPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        Spectator,
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn ui_example(
    mut contexts: EguiContexts,
) {
    egui::SidePanel::left("left")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Left fixed panel");
        });

    egui::SidePanel::right("right")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Right resizeable panel");

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    egui::Window::new("Movable Window").show(contexts.ctx_mut(), |ui| {
        ui.label("Move me!");
    });

    egui::Window::new("Immovable Window")
        .movable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("I can't be moved :(");
        });
}