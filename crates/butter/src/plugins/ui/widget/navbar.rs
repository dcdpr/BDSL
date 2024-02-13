use bevy_ecs::system::{SystemParam, SystemState};
use bevy_egui::egui::{self, Vec2};
use rfd::FileDialog;

use super::{RootWidgetSystem, UiWidgetSystemExt as _, WidgetSystem};
use crate::{plugins::ui::LastLoadPath, prelude::*};

#[derive(SystemParam)]
pub(in crate::plugins::ui) struct NavBar;

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
        egui::TopBottomPanel::top("navbar")
            .show_separator_line(false)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.set_height(40.);
                    ui.style_mut().spacing.button_padding = Vec2::splat(10.);
                    ui.add_system::<LoadButton>(world, "load_button");
                });
            });
    }
}

#[derive(SystemParam)]
struct LoadButton<'w> {
    load_path: ResMut<'w, LastLoadPath>,
}

impl WidgetSystem for LoadButton<'_> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        _: Self::Args,
    ) -> Self::Output {
        let LoadButton { mut load_path } = state.get_mut(world);

        if ui.button("Load Breadboard…").clicked() {
            if let Some(file) = FileDialog::new()
                .set_title("Open Breadboard File")
                .add_filter("breadboard", &["bnb"])
                .set_directory(&*load_path)
                .pick_file()
            {
                **load_path = file;
            }
        }
    }
}
