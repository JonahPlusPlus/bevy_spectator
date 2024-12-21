//! Handles focus / input conflicts between `bevy_egui` and this crate.
//!
//! ## Usage
//!
//! Enable the `bevy_egui` feature of this crate.
//!
//! Ensure [`bevy_egui::EguiPlugin`] is added to your app.
//!
//! *Note: this may be automatically done by `bevy_inspector_egui` plugins for example.*

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiSet};

/// A `Resource` for determining whether [`crate::Spectator`]s should handle input.
///
/// To check focus state it is recommended to call
///
/// [`EguiFocusState::wants_focus`]
///
/// which only returns true if egui wanted focus in both this frame and the previous frame.
///
#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiFocusState {
    /// Whether egui wants focus this frame
    pub current_frame_wants_focus: bool,
    /// Whether egui wanted focus in the previous frame
    pub previous_frame_wanted_focus: bool,
}

impl EguiFocusState {
    /// The default method for checking focus.
    pub fn wants_focus(&self) -> bool {
        self.previous_frame_wanted_focus && self.current_frame_wants_focus
    }
}

pub(crate) struct EguiFocusPlugin;

impl Plugin for EguiFocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiFocusState>();
    }

    fn finish(&self, app: &mut App) {
        if app.is_plugin_added::<EguiPlugin>() {
            app.add_systems(
                PostUpdate,
                check_egui_wants_focus.after(EguiSet::InitContexts),
            );
        } else {
            warn!("EguiPlugin not added, no focus checking will occur.");
        }
    }
}

fn check_egui_wants_focus(
    mut contexts: EguiContexts,
    mut focus_state: ResMut<EguiFocusState>,
    windows: Query<Entity, With<Window>>,
) {
    let egui_wants_focus_this_frame = windows.iter().any(|window| {
        let ctx = contexts.ctx_for_entity_mut(window);
        ctx.wants_pointer_input() || ctx.wants_keyboard_input() || ctx.is_pointer_over_area()
    });

    focus_state.set_if_neq(EguiFocusState {
        previous_frame_wanted_focus: focus_state.current_frame_wants_focus,
        current_frame_wants_focus: egui_wants_focus_this_frame,
    });
}
