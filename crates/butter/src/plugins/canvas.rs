mod affordance;
mod breadboard;
mod connection;
mod place;
mod shared;

use crate::prelude::*;

use super::schedule::AppSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum CanvasSet {
    Breadboard,
    Place,
    Affordance,
    Connection,
}

/// Render the breadboard canvas.
pub(crate) struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                CanvasSet::Breadboard,
                CanvasSet::Place,
                CanvasSet::Affordance,
                CanvasSet::Connection,
            )
                .chain()
                .in_set(AppSet::EntityUpdates),
        )
        .add_plugins((
            breadboard::BreadboardPlugin,
            place::PlacePlugin,
            affordance::AffordancePlugin,
            connection::ConnectionPlugin,
        ));
    }
}
