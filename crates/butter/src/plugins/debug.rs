use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};

use crate::prelude::*;

use super::schedule::AppSet;

/// Generic debugging utilities.
pub(crate) struct DebugPlugin {
    pub enable: bool,
    pub ambiguity_detection: bool,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            enable: true,
            ambiguity_detection: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.enable {
            app.add_systems(Update, print_position.after(AppSet::EntityUpdates));
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

/// Log the [`Entity`] ID and translation of each entity with a [`Transform`] component.
fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in &query {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}
