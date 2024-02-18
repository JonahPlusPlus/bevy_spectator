#![doc = include_str!("../README.md")]

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

/// A marker `Component` for spectating cameras.
///
/// ## Usage
/// With the `init` feature:
/// - Add it to a single entity to mark it as a spectator.
/// - `init` will then find that entity and mark it as the active spectator in [`SpectatorSettings`].
///
/// (If there isn't a single [`Spectator`] (none or multiple, instead of one), there won't be an active spectator selected by the `init` feature.)
///
/// Without the `init` feature:
/// - Add it to entities to mark spectators.
/// - Manually alter [`SpectatorSettings`] to set the active spectator.
#[derive(Component)]
pub struct Spectator;

/// A `Plugin` for spectating your scene.
pub struct SpectatorPlugin;

impl Plugin for SpectatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpectatorSettings>();

        #[cfg(feature = "init")]
        app.add_systems(PostStartup, spectator_init);

        app.add_systems(Update, spectator_update);
    }
}

#[cfg(feature = "init")]
fn spectator_init(
    cameras: Query<Entity, With<Spectator>>,
    mut settings: ResMut<SpectatorSettings>,
) {
    use bevy::ecs::query::QuerySingleError;

    if settings.active_spectator.is_none() {
        settings.active_spectator = match cameras.get_single() {
            Ok(a) => Some(a),
            Err(QuerySingleError::NoEntities(_)) => {
                warn!("Failed to find a Spectator; Active camera will remain unset.");
                None
            }
            Err(QuerySingleError::MultipleEntities(_)) => {
                warn!("Found more than one Spectator; Active camera will remain unset.");
                None
            }
        };
    }
}

#[allow(clippy::too_many_arguments)]
fn spectator_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut settings: ResMut<SpectatorSettings>,
    mut windows: Query<(&mut Window, Option<&PrimaryWindow>)>,
    mut camera_transforms: Query<&mut Transform, With<Spectator>>,
    mut focus: Local<bool>,
) {
    let Some(camera_id) = settings.active_spectator else {
        motion.clear();
        return;
    };

    let Ok(mut camera_transform) = camera_transforms.get_mut(camera_id) else {
        error!("Failed to find camera for active camera entity ({camera_id:?})");
        settings.active_spectator = None;
        motion.clear();
        return;
    };

    let mut window = match settings.active_window {
        Some(active) => {
            let Ok((window, _)) = windows.get_mut(active) else {
                error!("Failed to find active window ({active:?})");
                settings.active_window = None;
                motion.clear();
                return;
            };

            window
        }
        None => {
            let Some((window, _)) = windows.iter_mut().find(|(_, primary)| primary.is_some())
            else {
                panic!("No primary window found!");
            };

            window
        }
    };

    let mut set_focus = |focused: bool| {
        *focus = focused;
        if !settings.orthographic {
            let grab_mode = match focused {
                true => CursorGrabMode::Confined,
                false => CursorGrabMode::None,
            };
            window.cursor.grab_mode = grab_mode;
            window.cursor.visible = !focused;
        }
    };

    if keys.just_pressed(KeyCode::Escape) {
        set_focus(false);
    } else if buttons.just_pressed(MouseButton::Left) {
        set_focus(true);
    }

    // When in orthographic mode, mouse capturing is disabled. Movement should therefore always be enabled.
    if *focus || settings.orthographic {
        // rotation
        if !settings.orthographic {
            let mouse_delta = {
                let mut total = Vec2::ZERO;
                for d in motion.read() {
                    total += d.delta;
                }
                total
            };

            let mouse_x = -mouse_delta.x * settings.sensitivity;
            let mouse_y = -mouse_delta.y * settings.sensitivity;

            let mut dof: Vec3 = camera_transform.rotation.to_euler(EulerRot::YXZ).into();

            dof.x += mouse_x;
            // At 90 degrees, yaw gets misinterpreted as roll. Making 89 the limit fixes that.
            dof.y = (dof.y + mouse_y).clamp(-89f32.to_radians(), 89f32.to_radians());
            dof.z = 0f32;

            camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, dof.x, dof.y, dof.z);
        }

        // translation
        {
            let forward = if keys.pressed(KeyCode::KeyW) {
                1f32
            } else {
                0f32
            };

            let backward = if keys.pressed(KeyCode::KeyS) {
                1f32
            } else {
                0f32
            };

            let right = if keys.pressed(KeyCode::KeyD) {
                1f32
            } else {
                0f32
            };

            let left = if keys.pressed(KeyCode::KeyA) {
                1f32
            } else {
                0f32
            };

            let up_cond = if !settings.orthographic {
                keys.pressed(KeyCode::Space)
            } else {
                keys.pressed(KeyCode::KeyW)
            };
            let up = if up_cond { 1f32 } else { 0f32 };

            let down_cond = if !settings.orthographic {
                keys.pressed(KeyCode::ControlLeft)
            } else {
                keys.pressed(KeyCode::KeyS)
            };
            let down = if down_cond { 1f32 } else { 0f32 };

            let speed = if keys.pressed(KeyCode::ShiftLeft) {
                settings.alt_speed
            } else {
                settings.base_speed
            };

            let delta_axial = if settings.orthographic {
                0.0
            } else {
                (forward - backward) * speed
            };
            let delta_lateral = (right - left) * speed;
            let delta_vertical = (up - down) * speed;

            let mut forward = Vec3::from(camera_transform.forward());
            forward.y = 0f32;
            forward = forward.normalize_or_zero(); // fly fast even when look down/up

            let mut right = Vec3::from(camera_transform.right());
            right.y = 0f32; // more of a sanity check
            let up = Vec3::Y;

            let result = forward * delta_axial + right * delta_lateral + up * delta_vertical;

            camera_transform.translation += result * time.delta_seconds();
        }
    }

    motion.clear();
}

/// A `Resource` for controlling [`Spectator`]s.
#[derive(Resource)]
pub struct SpectatorSettings {
    /// The `Entity` of the active [`Spectator`]. (Default: `None`)
    ///
    /// Use this to control which [`Spectator`] you are using.
    ///
    /// If the `init` feature is enabled, `None` will update to a single, marked camera.
    ///
    /// Setting to `None` will disable the spectator mode.
    pub active_spectator: Option<Entity>,
    /// The `Entity` of the active `Window`. (Default: `None`)
    ///
    /// Use this to control which `Window` will grab your mouse/hide the cursor.
    ///
    /// If `None`, the primary window will be used.
    pub active_window: Option<Entity>,
    /// The base speed of the active [`Spectator`]. (Default: `10.0`)
    ///
    /// Use this to control how fast the [`Spectator`] normally moves.
    pub base_speed: f32,
    /// The alternate speed of the active [`Spectator`]. (Default: `50.0`)
    ///
    /// Use this to control how fast the [`Spectator`] moves when you hold `Shift`.
    pub alt_speed: f32,
    /// The camera sensitivity of the active [`Spectator`]. (Default: `0.001`)
    ///
    /// Use this to control how fast the [`Spectator`] turns when you move the mouse.
    pub sensitivity: f32,
    /// Use a control scheme more fit for orthographic (2D) rendering
    ///
    /// Disables mouse capturing and hiding, prevents moving along z-axis and uses W and S for y-axis movement
    pub orthographic: bool,
}

impl Default for SpectatorSettings {
    fn default() -> Self {
        Self {
            active_spectator: None,
            active_window: None,
            base_speed: 10.0,
            alt_speed: 50.0,
            sensitivity: 0.001,
            orthographic: false,
        }
    }
}
