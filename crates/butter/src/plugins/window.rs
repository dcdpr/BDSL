use bevy_a11y::AccessibilityPlugin;
use bevy_window::{PresentMode, Window};
use bevy_winit::{WinitPlugin, WinitSettings};

use crate::prelude::*;

/// Window Management.
pub(crate) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.945, 0.945, 0.941)))
            .insert_resource(WinitSettings::desktop_app())
            .add_plugins((
                AccessibilityPlugin,
                bevy_window::WindowPlugin {
                    primary_window: Some(Window {
                        title: "Butter: A Buttery Smooth Breadboarding UI.".into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                },
                WinitPlugin::default(),
            ));
    }
}
