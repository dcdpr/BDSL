#![allow(elided_lifetimes_in_paths, clippy::needless_pass_by_value)]

mod plugins;
pub(crate) mod prelude;

use plugins::{
    AssetManagementPlugin, BevyPlugin, DebugPlugin, DesignTokensPlugin, InputPlugin,
    SchedulePlugin, StartupPlugin, UiPlugin, WindowPlugin,
};
use prelude::*;

pub struct Config {
    pub debug: bool,
}

pub fn run(config: Config) {
    let Config { debug } = config;

    App::new()
        .add_plugins((
            // Loaded first to make sure we capture all following traces.
            DebugPlugin {
                trace: debug,
                ambiguity_detection: debug,
            },
            AssetManagementPlugin,
            BevyPlugin,
            UiPlugin,
            DesignTokensPlugin,
            InputPlugin,
            SchedulePlugin,
            StartupPlugin,
            WindowPlugin,
        ))
        .run();
}
