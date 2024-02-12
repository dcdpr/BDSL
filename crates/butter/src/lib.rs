#![allow(elided_lifetimes_in_paths, clippy::needless_pass_by_value)]

mod plugins;
pub(crate) mod prelude;

use plugins::{
    AssetManagementPlugin, BevyPlugin, DebugPlugin, InputPlugin, SchedulePlugin, StartupPlugin,
    UiPlugin, WindowPlugin,
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
            DebugPlugin {
                trace: debug,
                ambiguity_detection: debug,
            },
            InputPlugin,
            SchedulePlugin,
            StartupPlugin,
            UiPlugin,
            WindowPlugin,
        ))
        .run();
}