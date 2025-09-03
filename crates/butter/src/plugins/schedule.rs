use crate::prelude::*;

/// The state of the application, which allows for loading assets before the app is "running".
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub(crate) enum AppState {
    #[default]
    Startup,
    // Running,
}

/// The default system set configuration.
///
/// These sets are run in-order, and commands are flushed after the `DespawnEntities` set of
/// systems have run.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum AppSet {
    DespawnEntities,
    UserInput,
    EntityUpdates,
}

pub(crate) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                AppSet::DespawnEntities,
                // Flush commands (i.e. 'apply_deferred runs)
                AppSet::UserInput,
                AppSet::EntityUpdates,
            )
                .chain(),
        )
        .init_state::<AppState>()
        .add_systems(
            Update,
            ApplyDeferred
                .after(AppSet::DespawnEntities)
                .before(AppSet::UserInput),
        );
    }
}
