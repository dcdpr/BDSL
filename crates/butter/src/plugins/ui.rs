mod widget;

use bevy_egui::{egui, EguiContexts, EguiPlugin};

use self::widget::{navbar::NavBar, WorldWidgetSystemExt};
use crate::prelude::*;

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, configure)
            .add_systems(Update, render);
    }
}

#[instrument(level = "trace", skip_all)]
fn configure(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

/// Main rendering system for the UI layer.
///
/// This system is called on every runtime `tick`, as [`bevy_egui`] is an intermediate mode GUI
/// library, which means the UI is redrawn on every frame.
///
/// Because of this, a [`widget`] layer is added to provide better integration with Bevy systems,
/// to avoid having one giant system implementation to render all the UI at once.
///
/// Specifically, see [`widget::WidgetSystem`] for more details.
#[instrument(level = "info", skip_all)]
fn render(world: &mut World) {
    world.root_widget_with::<NavBar>("navbar", ());
}
