use crate::prelude::*;

use super::schedule::AppSet;

/// Generic debugging utilities.
pub(crate) struct DebugPlugin {
    pub enable: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.enable {
            app.add_systems(Update, print_position.after(AppSet::EntityUpdates));
        }
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
