use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};

use crate::prelude::*;

/// Generic debugging utilities.
pub(crate) struct DebugPlugin {
    /// Enable tracing.
    pub trace: bool,

    /// Enable ECS system run order ambiguity detection.
    pub ambiguity_detection: bool,
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
            .add_directive("bevy_ecs=error".parse().unwrap());

        tracing_subscriber::fmt::fmt()
            .with_env_filter(filter)
            .with_span_events(FmtSpan::ENTER)
            .with_target(true)
            .with_line_number(true)
            .init();
    }

    #[cfg(not(feature = "trace"))]
    fn enable_tracing(&self) {}
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            trace: false,
            ambiguity_detection: false,
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
    }
}
