use bevy_ecs::query::QueryFilter;

use crate::prelude::*;

use super::debug::DebugComputedSize;

/// The computed size of a node in a tree.
///
/// The size can be one of three variants:
///
/// - pending
/// - inherited
/// - static
///
/// For any tree of nodes, the following invariants must hold:
///
/// - Each node (entity) in the tree *MUST* have the `ComputedSize` component attached.
/// - Each leaf node in the tree *CANNOT* have an inherited computed size.
///
/// There are many ways in which a computed size can be calculated, but one example is based on
/// [`bevy_text::TextLayoutInfo`], which provides its own computed size at the end of an update
/// cycle.
#[derive(Debug, Component, Default, Copy, Clone, Reflect, PartialEq)]
pub enum ComputedSize {
    /// A `Pending` computed size means the size will be known eventually, but is waiting on other
    /// data to be generated before the final size can be determined.
    ///
    /// For example, an entity with the `Text` component will get the computed size defined once
    /// [`bevy_text::TextPipeline::queue_text()`] runs, and the [`bevy_text::TextLayoutInfo`]
    /// component is added.
    #[default]
    Pending,

    /// An `Inherit` computed size means the size is inherited from the child nodes.
    ///
    /// It is an invalid state for leaf entities to have a computed size of `Inherit`.
    Inherit,

    /// A `Static` computed size means the size is known for this node, without the need to iterate
    /// into the node's children to calculate it.
    Static(Vec2),
}

impl ComputedSize {
    /// Applies a transformation to the computed size.
    ///
    /// If the size is set to `Inherit` or `Pending`, then no changes are made, otherwise takes
    /// into account the scale and rotation transformations and returns the new `Static` size
    /// value.
    pub fn transformed(self, transform: Transform) -> Self {
        let scale = match self {
            Self::Inherit | Self::Pending => return self,
            Self::Static(scale) => scale,
        };

        // Apply scaling to the sprite dimensions
        let scaled_dimensions = scale * transform.scale.truncate();

        // Calculate the rotated bounding box
        let rotation = transform.rotation;
        let corners = [
            Vec2::new(-scaled_dimensions.x / 2.0, -scaled_dimensions.y / 2.0),
            Vec2::new(scaled_dimensions.x / 2.0, -scaled_dimensions.y / 2.0),
            Vec2::new(scaled_dimensions.x / 2.0, scaled_dimensions.y / 2.0),
            Vec2::new(-scaled_dimensions.x / 2.0, scaled_dimensions.y / 2.0),
        ];

        // Rotate corners and find min/max for bounding box
        let rotated_corners: Vec<Vec2> = corners
            .iter()
            .map(|corner| (rotation * Vec3::new(corner.x, corner.y, 0.0)).truncate())
            .collect();

        let min_x = rotated_corners
            .iter()
            .map(|v| v.x)
            .reduce(f32::min)
            .unwrap_or(0.0);
        let max_x = rotated_corners
            .iter()
            .map(|v| v.x)
            .reduce(f32::max)
            .unwrap_or(0.0);
        let min_y = rotated_corners
            .iter()
            .map(|v| v.y)
            .reduce(f32::min)
            .unwrap_or(0.0);
        let max_y = rotated_corners
            .iter()
            .map(|v| v.y)
            .reduce(f32::max)
            .unwrap_or(0.0);

        ComputedSize::Static(Vec2::new(max_x - min_x, max_y - min_y))
    }
}

/// Additional padding that can be applied to a node.
///
/// This component can be added to any node, regardless of its `ComputedSize` value, e.g. a node
/// that inherits its computed size from its children, can also add an additional padding that is
/// applied to the calculated bounding box. When calculating the computed size, the padding added
/// by child nodes is taken into account as well.
///
/// Similarly, a node with a static computed size can add additional padding using this component.
#[derive(Component, Default, Copy, Clone)]
pub(crate) struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }
}

/// Grouped system parameters that exposes a [`Self::size_of(Entity)`] method allowing for
/// calculating the computed size of any node within a tree.
///
/// FIXME: While this is a nice level of abstraction, it causes issues with other systems that need
/// (e.g.) access to `&mut Transform`, which causes access conflicts as Bevy's (and Rust's)
/// borrowing rules prevent both mutable and immutable access to the same Component.
///
/// To work around this for now, a generic type parameter `T` is added, which is applied as a
/// filter to the `Query<&Transform>` system parameter, which allows e.g. a system as this to be
/// valid:
///
/// ```rust,ignore
/// fn system(
///     a: ComputedSizeParam<Without<Foo>>,
///     b: Query<&mut Transform, With<Foo>,
/// ) {}
/// ```
///
/// This is cumbersome and often not desired, though, so we'll likely have to find an alternative
/// solution.
#[derive(SystemParam)]
pub(crate) struct ComputedSizeParam<'w, 's, T: QueryFilter + 'static> {
    children: Query<'w, 's, &'static Children>,
    sizes: Query<'w, 's, &'static ComputedSize>,
    paddings: Query<'w, 's, &'static Padding>,
    transforms: Query<'w, 's, &'static Transform, T>,
}

impl<T: QueryFilter + 'static> ComputedSizeParam<'_, '_, T> {
    /// Return the calculated size of an `Entity`.
    ///
    /// This returns `Ok(None)` if the size is not known yet (i.e. the computed size is
    /// [`ComputedSize::Pending`].
    ///
    /// A future improvement could try to cache these results, but cache invalidation is tricky to
    /// get right, and the caching itself might be slower than walking the actual tree.
    #[instrument(level = "debug", skip(self))]
    pub fn size_of(&self, entity: Entity) -> Result<Option<Vec2>, Error> {
        self.calculate_size_for_entity(entity)
    }

    #[instrument(level = "trace", skip(self))]
    fn calculate_size_for_entity(&self, entity: Entity) -> Result<Option<Vec2>, Error> {
        // Any node can apply padding to its calculated size.
        //
        // If a node inherits its size from its children, then the padding is added to the final
        // calculated size of the children (including any padding added by the children).
        //
        // If a node has a static size, then the padding is directly applied to that size.
        let padding = self.paddings.get(entity).copied().unwrap_or_default();

        // Any node in the tree MUST have a `ComputedSize` component attached.
        let computed_size = self
            .sizes
            .get(entity)
            .map_err(|_| Error::MissingSize(entity))?;

        match computed_size {
            // A pending computing size is a valid variant, but we can't return any known sizes at
            // this point.
            ComputedSize::Pending => {
                trace!(?entity, "ComputedSize::Pending");
                return Ok(None);
            }

            // If the entity has a static computed size, we can return it as-is (with any optional
            // padding), without traversing the children of the node.
            ComputedSize::Static(size) => {
                trace!(?entity, ?size, "ComputedSize::Static");

                // If either x or y is zero, that implies that the node is not visible, and this is
                // considered an invalid state.
                if size.x == 0.0 || size.y == 0.0 {
                    return Err(Error::ZeroWidthOrHeight(entity, *size));
                }

                return Ok(Some(Vec2::new(
                    size.x + padding.left + padding.right,
                    size.y + padding.top + padding.bottom,
                )));
            }

            // Inherited computed sizes are calculated next.
            ComputedSize::Inherit => {}
        };

        // Initialize bounding box extremes.
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        // Iterate all children of the node, and calculate the bounding box of each of them. If the
        // node is inheriting its size, but has no children (e.g. it's a leaf node), then that's an
        // invalid state for the tree to be in.
        let children = self
            .children
            .get(entity)
            .map_err(|_| Error::InheritingLeafNode(entity))?;

        // Using Bevy's default hierarchy tooling, this case should never happen, but it might if a
        // child is removed manually without removing the accompanying tag from the parent.
        if children.is_empty() {
            return Err(Error::MissingChildren(entity))?;
        }

        // If the node has multiple children, we need to create a bounding box based on the
        // position on the canvas of all children, taking the bottom left and top right position of
        // all children combined.
        //
        // If the node has a single child, we can take its computed size, without the need to look
        // at the position on the canvas.
        let single_child = children.len() == 1;
        for &child in children.iter() {
            // If a child node's size is still pending, then we abort calculating its parent node
            // size as well.
            let Some(child_size) = self.calculate_size_for_entity(child)? else {
                return Ok(None);
            };

            if single_child {
                return Ok(Some(Vec2::new(
                    child_size.x + padding.left + padding.right,
                    child_size.y + padding.top + padding.bottom,
                )));
            }

            let transform = self
                .transforms
                .get(child)
                .map_err(|_| Error::MissingTransform(child))?;

            let translation = transform.translation.truncate();
            let bottom_left = translation - (child_size / 2.0);
            let top_right = translation + (child_size / 2.0);

            min_x = min_x.min(bottom_left.x);
            min_y = min_y.min(bottom_left.y);
            max_x = max_x.max(top_right.x);
            max_y = max_y.max(top_right.y);
        }

        if min_x == f32::INFINITY || max_x == f32::NEG_INFINITY {
            unreachable!("invalid state must have triggered error return");
        }

        // Adjust min and max values to include padding
        min_x -= padding.left;
        min_y -= padding.bottom;
        max_x += padding.right;
        max_y += padding.top;

        Ok(Some(Vec2::new(max_x - min_x, max_y - min_y)))
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("node expected to have computed size: {0:?}")]
    MissingSize(Entity),

    #[error("leaf node expected to have static computed size: {0:?}")]
    InheritingLeafNode(Entity),

    #[error("inheriting non-leaf node without children to inherit from: {0:?}")]
    MissingChildren(Entity),

    #[error("node must have `Transform` component: {0:?}")]
    MissingTransform(Entity),

    #[error("static computed size must have non-zero width/height (was: {1:?}): {0:?}")]
    ZeroWidthOrHeight(Entity, Vec2),
}

pub(crate) struct ComputedSizePlugin;

impl Plugin for ComputedSizePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ComputedSize>()
            .add_event::<ComputedSizeUpdatedEvent>()
            .add_systems(
                Update,
                (
                    computed_size_updated,
                    debug_computed_size_changed.run_if(resource_exists::<DebugComputedSize>),
                )
                    .run_if(|q: Query<(), Changed<ComputedSize>>| !q.is_empty())
                    .after(AppSet::EntityUpdates),
            );
    }
}

#[instrument(level = "debug", skip_all)]
pub(crate) fn debug_computed_size_changed(
    sizes: Query<(Entity, &ComputedSize), Changed<ComputedSize>>,
) {
    for (entity, size) in &sizes {
        debug!(?entity, ?size, "ComputedSize changed.");
    }
}

#[derive(Event)]
pub(crate) struct ComputedSizeUpdatedEvent {
    /// The source entity for which the computed size was updated.
    pub source: Entity,

    /// The tree of entities affected by this size update.
    ///
    /// These include all source ancestors that have their computed size set to `Inherit` stopping
    /// at the first ancestor that does not.
    pub ancestors: Vec<Entity>,
}

impl ComputedSizeUpdatedEvent {
    pub fn contains(&self, entity: Entity) -> bool {
        self.source == entity || self.ancestors.contains(&entity)
    }
}

/// Propagates computed size update events through the node tree hierarchy.
///
/// This function is called when the [`ComputedSize`] component of an [`Entity`] changes,
/// indicating that the visual representation of the entity or its layout requirements have been
/// updated. It ensures that any necessary updates or adjustments can be made in response to these
/// changes, particularly for entities that inherit or depend on the sizes of their descendants.
///
/// The function iterates over all entities that have had their `ComputedSize` changed, recursively
/// identifying all ancestors that inherit their size. Each identified source entity, along with
/// its ancestors affected by the size change, is then included in a [`ComputedSizeUpdatedEvent`]
/// and dispatched.
#[instrument(level = "trace", skip_all)]
pub(crate) fn computed_size_updated(
    mut writer: EventWriter<ComputedSizeUpdatedEvent>,
    changes: Query<Entity, Changed<ComputedSize>>,
    sizes: Query<&ComputedSize>,
    parents: Query<&Parent>,
) {
    for source in &changes {
        let mut ancestors: Vec<Entity> = vec![];

        find_ancestors(source, &mut ancestors, &sizes, &parents);
        writer.send(ComputedSizeUpdatedEvent { source, ancestors });
    }
}

fn find_ancestors(
    source: Entity,
    ancestors: &mut Vec<Entity>,
    sizes: &Query<&ComputedSize>,
    parents: &Query<&Parent>,
) {
    if let Ok(parent) = parents.get(source).map(Parent::get) {
        if let Ok(ComputedSize::Inherit) = sizes.get(parent) {
            ancestors.push(parent);
            find_ancestors(parent, ancestors, sizes, parents);
        }
    }
}
