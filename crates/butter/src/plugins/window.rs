use bevy::a11y::AccessibilityPlugin;
use bevy::window::{PresentMode, RequestRedraw, Window};
use bevy::winit::{WakeUp, WinitPlugin, WinitSettings};

use crate::prelude::*;

use super::canvas::{
    AffordanceCreatedEvent, BreadboardCreatedEvent, ConnectionCreated, PlaceCreatedEvent,
};

/// Window Management.
pub(crate) struct WindowPlugin;

#[derive(Resource, Default)]
pub(crate) struct ForceRedraw(pub bool);

impl ForceRedraw {
    pub fn set(&mut self) {
        self.0 = true;
    }

    pub fn reset(&mut self) -> bool {
        let doit = self.0;
        self.0 = false;
        doit
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.945, 0.945, 0.941)))
            .insert_resource(WinitSettings::desktop_app())
            .init_resource::<ForceRedraw>()
            .add_plugins((
                AccessibilityPlugin,
                bevy::window::WindowPlugin {
                    primary_window: Some(Window {
                        title: "Butter: A Buttery Smooth Breadboarding UI.".into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                },
                WinitPlugin::<WakeUp>::default(),
            ))
            .add_systems(
                Update,
                (canvas_redraw, force_redraw).before(AppSet::DespawnEntities),
            );
    }
}

fn canvas_redraw(
    mut redraw: EventWriter<RequestRedraw>,

    mut breadboard: EventReader<BreadboardCreatedEvent>,
    mut place: EventReader<PlaceCreatedEvent>,
    mut affordance: EventReader<AffordanceCreatedEvent>,
    mut connection: EventReader<ConnectionCreated>,
) {
    if breadboard.is_empty() && place.is_empty() && affordance.is_empty() && connection.is_empty() {
        return;
    }

    breadboard.clear();
    place.clear();
    affordance.clear();
    connection.clear();

    debug!("canvas_redraw");

    redraw.send(RequestRedraw);
}

fn force_redraw(mut redraw: EventWriter<RequestRedraw>, mut force_redraw: ResMut<ForceRedraw>) {
    if !force_redraw.reset() {
        return;
    }

    debug!("force_redraw");

    redraw.send(RequestRedraw);
}
