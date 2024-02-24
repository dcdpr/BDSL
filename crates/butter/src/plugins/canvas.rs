//! Canvas Plugin: Visualizing Breadboard Configurations
//!
//! Hosts the core functionality for rendering the dynamic visual representation of breadboards.
//! Through a set of sub-plugins:
//!
//! - [`BreadboardPlugin`]
//! - [`PlacePlugin`]
//! - [`AffordancePlugin`]
//! - [`ConnectionPlugin`]
//!
//! It orchestrates the visualization of the breadboard's components, enabling an intuitive and
//! interactive layout for users to explore and understand their designs. This plugin plays a
//! crucial role in bridging the gap between the abstract definitions of a breadboard DSL and their
//! tangible representation on the screen.
//!
//! For detailed information on individual parts of this plugin, please refer to the respective
//! documentation within this module.

mod affordance;
mod breadboard;
mod connection;
mod place;
mod shared;

use crate::prelude::*;

pub(crate) use affordance::AffordanceCreatedEvent;
pub(crate) use breadboard::BreadboardCreatedEvent;
pub(crate) use connection::ConnectionCreated;
pub(crate) use place::PlaceCreatedEvent;

use self::{
    affordance::AffordancePlugin, breadboard::BreadboardPlugin, connection::ConnectionPlugin,
    place::PlacePlugin,
};

/// Marker component for the root entity of the canvas.
///
/// This component is used to identify the main canvas entity within the ECS architecture. It
/// serves as a key identifier for systems and queries that need to interact with the canvas as a
/// whole, distinguishing it from other entities in the scene. Attaching this marker to an entity
/// effectively designates it as the central hub for breadboard visualization and interaction.
#[derive(Component)]
struct Canvas;

/// Represents the distinct stages of the canvas rendering process.
///
/// This enum categorizes the various system sets used within the `CanvasPlugin` to manage the
/// lifecycle and rendering logic of the canvas and its components. Each variant corresponds to a
/// specific phase in the canvas setup and update cycle, ensuring that systems are executed in the
/// correct order for proper visual representation and functionality.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum CanvasSet {
    Setup,
    Breadboard,
    Place,
    Affordance,
    Connection,
}

/// A plugin for rendering the breadboard canvas.
///
/// For a detailed overview of the plugin's architecture and functionalities, refer to the
/// module-level documentation.
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
            BreadboardPlugin,
            PlacePlugin,
            AffordancePlugin,
            ConnectionPlugin,
        ))
        .add_systems(
            Update,
            (
                spawn_canvas.run_if(run_once()),
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

/// Spawns the root canvas entity with essential components for rendering.
///
/// Initializes the canvas entity, equipping it with the `Canvas` marker component, default
/// visibility, and transform properties to set up the visual foundation for the breadboard and its
/// elements.
#[instrument(skip_all)]
fn spawn_canvas(mut cmd: Commands) {
    cmd.spawn((
        Canvas,
        VisibilityBundle::default(),
        TransformBundle::default(),
    ));
}

/// Ensures text entities have a computed size aligned with their text layout.
///
/// This function is responsible for adjusting the `ComputedSize` of text entities when there is a
/// change in their `TextLayoutInfo`. It's crucial for maintaining the visual accuracy of text
/// elements within the canvas, ensuring that their size matches the layout constraints.
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

/// Adjusts the computed size of entities in response to transformations.
///
/// This function recalculates the `ComputedSize` for entities undergoing scale or rotation
/// transformations. Such transformations alter the entity's bounding box, necessitating a
/// corresponding adjustment in the computed size to accurately reflect these changes.
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

/// Validates structural invariants within the canvas hierarchy.
///
/// This function ensures that every node within the canvas adheres to two key invariants:
/// 1. All nodes must possess both `Transform` and `ComputedSize` components to guarantee accurate
///    positioning and sizing within the canvas.
/// 2. Leaf nodes cannot inherit their size—they must have a statically defined `ComputedSize`.
///
/// Enforcing these invariants is essential for the integrity of the canvas layout, ensuring that
/// each node is correctly represented and interacts as expected within the overall structure.
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
