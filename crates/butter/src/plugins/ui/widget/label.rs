use bevy_ecs::system::{SystemParam, SystemState};
use bevy_egui::egui;

use super::WidgetSystem;
use crate::prelude::*;

#[derive(SystemParam)]
pub struct Label;

impl WidgetSystem for Label {
    type Args = ();
    type Output = ();

    #[instrument(name = "label", level = "info", skip_all)]
    fn system(_: &mut World, _: &mut SystemState<Self>, ui: &mut egui::Ui, _: Self::Args) {
        ui.label("Hello World!");
    }
}
