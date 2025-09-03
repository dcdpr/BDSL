use bevy::ecs::system::{SystemParam, SystemState};
use bevy_egui::egui::{self, Vec2};

use crate::{
    plugins::{canvas::ShowNumbersCheckbox, file_watcher::LoadButton},
    prelude::*,
    widget::RootWidgetSystem,
};

#[derive(SystemParam)]
pub(in crate::plugins::ui) struct NavBar;

impl RootWidgetSystem for NavBar {
    type Args = ();
    type Output = ();

    #[instrument(level = "trace", name = "navbar", skip_all)]
    fn system(
        world: &mut World,
        _: &mut SystemState<Self>,
        ctx: &mut egui::Context,
        _: Self::Args,
    ) {
        egui::TopBottomPanel::top("navbar")
            .show_separator_line(false)
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.set_height(40.);
                    ui.style_mut().spacing.button_padding = Vec2::splat(10.);
                    ui.add_system::<LoadButton>(world, "load_button");
                    ui.add_system::<ShowNumbersCheckbox>(world, "show_numbers");
                });
            });
    }
}
