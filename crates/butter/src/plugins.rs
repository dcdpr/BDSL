//! The plugin architecture allows for modular development and integration, enabling each aspect of
//! the application to be managed efficiently.

pub(super) mod asset_management;
pub(super) mod bevy;
pub(super) mod canvas;
pub(super) mod computed_size;
pub(super) mod debug;
pub(super) mod design_tokens;
pub(super) mod error_handler;
pub(super) mod file_watcher;
pub(super) mod input;
pub(super) mod inspector;
pub(super) mod rng;
pub(super) mod schedule;
pub(super) mod startup;
pub(super) mod ui;
pub(super) mod window;

pub(super) use asset_management::AssetManagementPlugin;
pub(super) use bevy::BevyPlugin;
pub(super) use canvas::CanvasPlugin;
pub(super) use computed_size::ComputedSizePlugin;
pub(super) use debug::DebugPlugin;
pub(super) use design_tokens::DesignTokensPlugin;
pub(super) use error_handler::ErrorHandlerPlugin;
pub(super) use file_watcher::FileWatcherPlugin;
pub(super) use input::InputPlugin;
pub(super) use inspector::InspectorPlugin;
pub(super) use rng::RngPlugin;
pub(super) use schedule::SchedulePlugin;
pub(super) use startup::StartupPlugin;
pub(super) use ui::UiPlugin;
pub(super) use window::WindowPlugin;
