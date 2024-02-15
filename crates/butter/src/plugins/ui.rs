mod navbar;

use bevy_egui::{
    egui::{self, Color32, CursorIcon, Visuals},
    EguiContexts, EguiPlugin, EguiSet,
};
use dtoken::types::color::Color;

use crate::{prelude::*, widget::WorldWidgetSystemExt as _};

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(
                PreUpdate,
                apply_base_theme
                    .run_if(run_once())
                    // From `bevy_egui` documentation:
                    //
                    // Systems that create Egui widgets should be run during the `CoreSet::Update`
                    // set, or after the `EguiSet::BeginFrame` system (which belongs to the
                    // `CoreSet::PreUpdate` set).
                    .after(EguiSet::BeginFrame),
            )
            .add_systems(Update, render);
    }
}

#[instrument(level = "trace", skip_all)]
fn apply_base_theme(tokens: Res<DesignTokens>, mut contexts: EguiContexts) {
    let v = &tokens.egui.visuals;
    let c = |c: Color| Color32::from_rgb(c.r, c.g, c.b);

    let old = contexts.ctx_mut().style().visuals.clone();
    contexts.ctx_mut().set_visuals(Visuals {
        override_text_color: Some(c(v.override_text_color)),
        hyperlink_color: c(v.hyperlink_color),
        faint_bg_color: c(v.faint_bg_color),
        extreme_bg_color: c(v.extreme_bg_color),
        code_bg_color: c(v.code_bg_color),
        warn_fg_color: c(v.warn_fg_color),
        error_fg_color: c(v.error_fg_color),
        window_fill: c(v.window_fill),
        panel_fill: c(v.panel_fill),
        window_stroke: egui::Stroke {
            color: c(v.window_stroke),
            ..old.window_stroke
        },
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: c(v.widgets.noninteractive),
                weak_bg_fill: c(v.widgets.noninteractive),
                bg_stroke: egui::Stroke {
                    color: c(tokens.colors.overlay_1),
                    ..default()
                },
                fg_stroke: egui::Stroke {
                    color: c(tokens.colors.text),
                    ..default()
                },
                ..old.widgets.noninteractive
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: c(v.widgets.inactive),
                weak_bg_fill: c(v.widgets.inactive),
                bg_stroke: egui::Stroke {
                    color: c(tokens.colors.overlay_1),
                    ..default()
                },
                fg_stroke: egui::Stroke {
                    color: c(tokens.colors.text),
                    ..default()
                },
                ..old.widgets.inactive
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: c(v.widgets.hovered),
                weak_bg_fill: c(v.widgets.hovered),
                bg_stroke: egui::Stroke {
                    color: c(tokens.colors.overlay_1),
                    ..default()
                },
                fg_stroke: egui::Stroke {
                    color: c(tokens.colors.text),
                    ..default()
                },
                ..old.widgets.inactive
            },
            active: egui::style::WidgetVisuals {
                bg_fill: c(v.widgets.active),
                weak_bg_fill: c(v.widgets.active),
                bg_stroke: egui::Stroke {
                    color: c(tokens.colors.overlay_1),
                    ..default()
                },
                fg_stroke: egui::Stroke {
                    color: c(tokens.colors.text),
                    ..default()
                },
                ..old.widgets.inactive
            },
            open: egui::style::WidgetVisuals {
                bg_fill: c(v.widgets.open),
                weak_bg_fill: c(v.widgets.open),
                bg_stroke: egui::Stroke {
                    color: c(tokens.colors.overlay_1),
                    ..default()
                },
                fg_stroke: egui::Stroke {
                    color: c(tokens.colors.text),
                    ..default()
                },
                ..old.widgets.inactive
            },
            ..default()
        },
        selection: egui::style::Selection {
            bg_fill: c(v.selection).linear_multiply(0.4),
            stroke: egui::Stroke {
                color: c(tokens.colors.overlay_1),
                ..old.selection.stroke
            },
        },
        window_shadow: egui::epaint::Shadow {
            color: c(v.window_shadow),
            ..old.window_shadow
        },
        popup_shadow: egui::epaint::Shadow {
            color: c(v.popup_shadow),
            ..old.popup_shadow
        },
        dark_mode: false,
        interact_cursor: Some(CursorIcon::PointingHand),
        ..default()
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
#[instrument(level = "trace", skip_all)]
fn render(world: &mut World) {
    world.root_widget_with::<navbar::NavBar>("navbar", ());
}
