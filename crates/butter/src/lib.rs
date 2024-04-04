#![allow(elided_lifetimes_in_paths, clippy::needless_pass_by_value)]

mod plugins;
pub(crate) mod prelude;
pub(crate) mod widget;

use plugins::{
    AssetManagementPlugin, BevyPlugin, CanvasPlugin, ComputedSizePlugin, DebugPlugin,
    DesignTokensPlugin, ErrorHandlerPlugin, FileWatcherPlugin, InputPlugin, InspectorPlugin,
    RngPlugin, SchedulePlugin, StartupPlugin, UiPlugin, WindowPlugin,
};
use prelude::*;

pub struct Config {
    pub debug: bool,
}

pub fn run(config: Config) {
    let Config { debug } = config;

    App::new()
        .add_plugins((
            AssetManagementPlugin,
            BevyPlugin,
            // Loaded first to make sure we capture all following traces.
            DebugPlugin {
                trace: debug,
                ambiguity_detection: debug,
                computed_size_changes: debug,
                draw_gizmos: debug,
                infinite_zoom: debug,
            },
            UiPlugin,
            // Separate from `DebugPlugin` as it relies on running after `BevyPlugin`.
            InspectorPlugin { enable: debug },
            DesignTokensPlugin,
            InputPlugin,
            SchedulePlugin,
            StartupPlugin,
            WindowPlugin,
            FileWatcherPlugin,
            CanvasPlugin,
            RngPlugin,
            ComputedSizePlugin,
            ErrorHandlerPlugin,
        ))
        .run();
}
