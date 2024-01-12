#![allow(elided_lifetimes_in_paths, clippy::needless_pass_by_value)]

mod plugins;
pub(crate) mod prelude;

use bevy_window::PresentMode;
use plugins::{
    asset_management::AssetManagementPlugin, debug::DebugPlugin, schedule::SchedulePlugin,
};
use prelude::*;

pub struct Config {
    pub debug: bool,
}

pub fn run(config: Config) {
    let Config { debug } = config;

    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .add_plugins((
            AccessibilityPlugin,
            InputPlugin,
            TransformPlugin,
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Butter: A Buttery Smooth Breadboarding UI.".into(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            },
            WinitPlugin::default(),
        ))
        // User defined plugins.
        .add_plugins((
            AssetManagementPlugin,
            DebugPlugin { enable: debug },
            SchedulePlugin,
        ))
        .run();
}
