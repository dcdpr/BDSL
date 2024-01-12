use crate::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, configure)
            .add_systems(Update, hello_world);
    }
}

#[instrument(level = "trace", skip_all)]
fn configure(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

#[instrument(level = "info", skip_all)]
fn hello_world(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
