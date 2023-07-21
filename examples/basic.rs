use bevy::prelude::*;
use bevy_spectator::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpectatorPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), Spectator));
}
