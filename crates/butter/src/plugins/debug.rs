use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};

use crate::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct DebugComputedSize;

/// Generic debugging utilities.
pub(crate) struct DebugPlugin {
    /// Enable tracing.
    pub trace: bool,

    /// Enable ECS system run order ambiguity detection.
    pub ambiguity_detection: bool,

    /// Enable debugging of changes in canvas node computed sizes.
    pub computed_size_changes: bool,
}
impl DebugPlugin {
    #[cfg(feature = "trace")]
    fn enable_tracing(&self) {
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .with_env_var("LOG")
            .from_env_lossy()
            .add_directive("bevy_ecs=error".parse().unwrap())
            .add_directive("bevy_render=error".parse().unwrap())
            .add_directive("wgpu_core=error".parse().unwrap())
            .add_directive("wgpu_hal=error".parse().unwrap())
            .add_directive("bevy_time::virt=error".parse().unwrap())
            .add_directive("bevy_mod_raycast=error".parse().unwrap())
            .add_directive("naga=error".parse().unwrap());

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_span_events(FmtSpan::ENTER)
            .with_target(true)
            .with_line_number(true)
            .init();
    }

    #[cfg(not(feature = "trace"))]
    fn enable_tracing(&self) {
        tracing::warn!("`trace` option enabled, but binary built without `trace` feature")
    }
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            trace: false,
            ambiguity_detection: false,
            computed_size_changes: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.trace {
            self.enable_tracing();
        }

        if self.ambiguity_detection {
            app.edit_schedule(Update, |schedule| {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Warn,
                    ..default()
                });
            });
        };

        if self.computed_size_changes {
            app.init_resource::<DebugComputedSize>();
        }
    }
}
