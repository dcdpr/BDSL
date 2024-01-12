use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::EguiInput;

use super::schedule::{AppState, LoadState};

pub(crate) struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(
                Update,
                foo.run_if(in_state(LoadState::Loading).and_then(in_state(AppState::Startup))),
            )
            .add_plugins(bevy_egui::EguiPlugin);
    }
}

fn foo(mut drop_events: EventReader<FileDragAndDrop>) {
    for event in drop_events.read() {
        dbg!(event);
    }
}
