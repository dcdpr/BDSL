use crate::prelude::*;

use super::{affordance::AffordanceCreatedEvent, CanvasSet};

/// Manage *affordances* in a place.
pub(super) struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectionCreated>().add_systems(
            Update,
            create
                .run_if(on_event::<AffordanceCreatedEvent>())
                .in_set(CanvasSet::Connection),
        );
    }
}

/// Marker component for connection entities.
#[derive(Component, Default)]
struct Connection;

/// Bundle of required components for place entities.
#[derive(Bundle, Default)]
struct ConnectionBundle {
    marker: Connection,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    size: ComputedSize,
}

#[derive(Event)]
#[allow(dead_code)]
pub(crate) struct ConnectionCreated {
    pub entity: Entity,
    pub target_place: Name,
}

#[instrument(skip_all)]
fn create(
    mut cmd: Commands,
    mut affordances: EventReader<AffordanceCreatedEvent>,
    mut created: EventWriter<ConnectionCreated>,
) {
    for &AffordanceCreatedEvent {
        entity,
        ref connections,
        ..
    } in affordances.read()
    {
        for ast::Connection { target_place, .. } in connections.clone() {
            let _span = info_span!("spawn", affordance = ?entity, target = %target_place).entered();

            // TODO: Disabled for now, as it results in `ComputedSize::Pending`, which prevents
            // the board from becoming visible.
            //
            // let entity = cmd
            //     .spawn(ConnectionBundle::default())
            //     .set_parent(entity)
            //     .id();
            //
            // created.send(ConnectionCreated {
            //     entity,
            //     target_place: target_place.into(),
            // });
        }
    }
}
