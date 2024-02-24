use bevy_a11y::AccessibilityPlugin;
use bevy_window::{PresentMode, RequestRedraw, Window};
use bevy_winit::{WinitPlugin, WinitSettings};

use crate::prelude::*;

use super::canvas::{
    AffordanceCreatedEvent, BreadboardCreatedEvent, ConnectionCreated, PlaceCreatedEvent,
};

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
            ))
            .add_systems(Update, force_redraw.before(AppSet::DespawnEntities));
    }
}

fn force_redraw(
    mut redraw: EventWriter<RequestRedraw>,

    mut breadboard: EventReader<BreadboardCreatedEvent>,
    mut place: EventReader<PlaceCreatedEvent>,
    mut affordance: EventReader<AffordanceCreatedEvent>,
    mut connection: EventReader<ConnectionCreated>,
) {
    if breadboard.is_empty() && place.is_empty() && affordance.is_empty() && connection.is_empty() {
        return;
    };

    breadboard.clear();
    place.clear();
    affordance.clear();
    connection.clear();

    debug!("force_redraw");

    redraw.send(RequestRedraw);
}
