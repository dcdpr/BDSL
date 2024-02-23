mod affordance;
mod breadboard;
mod connection;
mod place;
mod shared;

use bevy_text::TextLayoutInfo;

use crate::prelude::*;

use super::{computed_size::ComputedSize, schedule::AppSet};

pub(crate) use affordance::AffordanceCreated;
pub(crate) use breadboard::BreadboardCreated;
pub(crate) use connection::ConnectionCreated;
pub(crate) use place::PlaceCreated;

/// Marker component for the root entity of the canvas.
#[derive(Component)]
struct Canvas;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum CanvasSet {
    Setup,
    Breadboard,
    Place,
    Affordance,
    Connection,
}

// TODO: Add a `Canvas` entity, under which all breadboards etc live?

/// Render the breadboard canvas.
pub(crate) struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                CanvasSet::Setup,
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
        ))
        .add_systems(
            Update,
            (
                setup.run_if(run_once()),
                update_text_computed_size.run_if(
                    |q: Query<(), (With<ComputedSize>, Changed<TextLayoutInfo>)>| !q.is_empty(),
                ),
                update_transformed_computed_size
                    .run_if(|q: Query<(), (With<ComputedSize>, Changed<Transform>)>| !q.is_empty()),
                ensure_node_compliance,
            )
                .chain()
                .in_set(CanvasSet::Setup),
        );
    }
}

#[instrument(skip_all)]
fn setup(mut cmd: Commands) {
    cmd.spawn((
        Canvas,
        VisibilityBundle::default(),
        TransformBundle::default(),
    ));
}

#[instrument(skip_all)]
fn update_text_computed_size(
    mut sizes: Query<(Entity, &mut ComputedSize, &TextLayoutInfo), Changed<TextLayoutInfo>>,
) {
    for (entity, mut size, layout) in sizes.iter_mut() {
        // The logical size of a text node can be zero, which we interpret as "unknown".
        if layout.logical_size == Vec2::ZERO {
            if size.as_ref() == &ComputedSize::Inherit {
                continue;
            }
        }

        let old = *size.as_ref();
        if size.set_if_neq(ComputedSize::Static(layout.logical_size)) {
            let new = size.as_ref();
            debug!(?entity, ?old, ?new, "Updated ComputedSize text component");
        }
    }
}

#[instrument(skip_all)]
fn update_transformed_computed_size(
    mut sizes: Query<(Entity, &mut ComputedSize, &Transform), Changed<Transform>>,
) {
    for (entity, mut size, transform) in sizes.iter_mut() {
        let old = *size.as_ref();
        if size.set_if_neq(old.transformed(*transform)) {
            let new = size.as_ref();
            debug!(
                ?entity,
                ?old,
                ?new,
                "Updated transformed ComputedSize component"
            );
        }
    }
}

#[instrument(level = "trace", skip_all)]
fn ensure_node_compliance(
    root: Query<Entity, With<Canvas>>,
    children: Query<&Children>,
    nodes: Query<(Option<&Children>, &ComputedSize), With<Transform>>,
) {
    let Ok(canvas) = root.get_single() else {
        let component = std::any::type_name::<Canvas>();
        error!(%component, "Canvas does not have single root entity with marker component.");
        return;
    };

    for node in children.iter_descendants(canvas) {
        let Ok((children, computed_size)) = nodes.get(node) else {
            error!(
                ?node,
                "Nodes must have `Transform` and `ComputedSize` components."
            );
            continue;
        };

        let is_leaf = children.map_or(true, |c| c.is_empty());
        if is_leaf && matches!(computed_size, ComputedSize::Inherit) {
            error!(?node, "Leaf nodes must have known computed size.");
        }
    }
}
