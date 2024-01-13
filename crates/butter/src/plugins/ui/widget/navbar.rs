use bevy_ecs::system::{SystemParam, SystemState};
use bevy_egui::egui;

use super::{label::Label, RootWidgetSystem, UiWidgetSystemExt as _};
// use super::label;
use crate::prelude::*;

#[derive(SystemParam)]
pub struct NavBar;

impl RootWidgetSystem for NavBar {
    type Args = ();
    type Output = ();

    #[instrument(name = "navbar", level = "info", skip_all)]
    fn system(
        world: &mut World,
        _: &mut SystemState<Self>,
        ctx: &mut egui::Context,
        _: Self::Args,
    ) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.add_system_with::<Label>(world, "hello_world", ());
        });
    }
}
