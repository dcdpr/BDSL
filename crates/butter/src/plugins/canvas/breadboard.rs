//! Breadboard Plugin: Managing Breadboard Entities within the Canvas
//!
//! This module introduces the [`BreadboardPlugin`], a component of the larger
//! [`CanvasPlugin`](super::CanvasPlugin) ecosystem focused on the creation and management of
//! breadboard entities. It leverages event-driven systems to dynamically respond to changes and
//! user interactions, specifically handling the instantiation of breadboard entities upon the
//! loading of DSL files and adjusting their visibility within the canvas.
//!
//! For detailed information on individual parts of this plugin, please refer to the respective
//! documentation within this module.

use crate::{plugins::file_watcher::FileLoadedEvent, prelude::*};

use super::{Canvas, CanvasSet};

/// Render the breadboard on the window canvas.
pub(super) struct BreadboardPlugin;

impl Plugin for BreadboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowNumbers(true))
            .add_event::<BreadboardCreatedEvent>()
            .add_systems(
                Update,
                (
                    spawn.run_if(on_event::<FileLoadedEvent>()),
                    make_visible.run_if(|q: Query<&Visibility, With<Breadboard>>| {
                        q.iter().any(|v| v == Visibility::Hidden)
                    }),
                )
                    .chain()
                    .in_set(CanvasSet::Breadboard),
            );
    }
}

/// Marker component for breadboard entities.
#[derive(Component, Default)]
pub(crate) struct Breadboard;

/// Bundle of required components for breadboard entities.
#[derive(Bundle)]
pub(super) struct BreadboardBundle {
    name: Name,
    marker: Breadboard,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    size: ComputedSize,
}

impl Default for BreadboardBundle {
    fn default() -> Self {
        Self {
            size: ComputedSize::Inherit,
            name: default(),
            marker: default(),
            visibility: default(),
            transform: default(),
        }
    }
}

impl BreadboardBundle {
    pub(super) fn new(name: Name) -> Self {
        Self {
            name,
            visibility: VisibilityBundle {
                // Start off as hidden, until all children are spawned and positioned correctly.
                // This prevents any visual glitches during the initial render.
                visibility: Visibility::Hidden,
                ..default()
            },
            ..default()
        }
    }
}

/// An event signaling the creation of a breadboard entity.
///
/// This event is dispatched when a new breadboard entity is successfully created, carrying with it
/// the entity's identifier and a list of its constituent places derived from the parsed DSL. It
/// serves as a notification mechanism for other systems to react to the introduction of a new
/// breadboard into the scene, enabling subsequent initialization or update processes related to
/// the breadboard's components.
#[derive(Event)]
pub(crate) struct BreadboardCreatedEvent {
    pub entity: Entity,
    pub places: Vec<ast::Place>,
}

/// Spawns a new breadboard entity based on the loaded file.
///
/// Processes each [`FileLoadedEvent`], attempting to parse the file contents into a breadboard DSL
/// structure. If parsing succeeds, any existing breadboard with the same name is removed from the
/// canvas to make room for the new one. The new breadboard entity is then created, with visual
/// variations seeded by its name to ensure a unique, yet consistent, hand-drawn appearance.
///
/// Finally, a [`BreadboardCreatedEvent`] is emitted to signal the successful creation of the
/// breadboard.
#[instrument(skip_all)]
fn spawn(
    mut cmd: Commands,
    boards: Query<(Entity, &Name), With<Breadboard>>,
    canvas: Query<Entity, With<Canvas>>,
    mut loaded: EventReader<FileLoadedEvent>,
    mut created: EventWriter<BreadboardCreatedEvent>,
) {
    for FileLoadedEvent { name, contents } in loaded.read() {
        let span = info_span!("spawn", %name, breadboard = field::Empty).entered();

        let Ok(ast::Breadboard { places, .. }) = parser::parse(contents) else {
            // TODO: Trigger `alert` widget.
            continue;
        };

        let name = Name::new(name.to_owned());

        // Despawn existing breadboard with matching names.
        boards
            .iter()
            .filter_map(|(entity, n)| (n == &name).then_some(entity))
            .for_each(|entity| cmd.entity(entity).despawn_recursive());

        // Random elements of the breadboard (slight font changes, underline changes, etc, to give
        // it more of a hand-drawn feel) are seeded based on the name of the breadboard, this
        // ensures consistent rendering between sessions.
        let seed: u64 = name.bytes().fold(0, |acc, n| acc + n as u64);

        // Spawn new breadboard entity.
        let entity = cmd
            .spawn(BreadboardBundle::new(name))
            .insert(RngComponent::with_seed(seed))
            .set_parent(canvas.single())
            .id();

        span.record("breadboard", format!("{entity:?}"));

        // Trigger creation event.
        created.send(BreadboardCreatedEvent { entity, places });
    }
}

/// Makes hidden breadboards visible if they have a computed size.
///
/// Iterates over breadboards that are currently not visible and checks if they have a valid
/// computed size using the `ComputedSizeParam` system parameter. Breadboards with a determined
/// size are then made visible. This ensures that only breadboards ready for display (i.e., those
/// with calculated dimensions) are shown, aiding in maintaining a clean and coherent visual
/// presentation of the canvas.
#[instrument(skip_all)]
fn make_visible(
    mut breadboards: Query<(Entity, &mut Visibility), With<Breadboard>>,
    sizes: ComputedSizeParam<()>,
) {
    // Iterate all breadboards that are currently hidden.
    for (entity, mut visibility) in breadboards
        .iter_mut()
        .filter(|(_, vis)| vis.as_ref() == Visibility::Hidden)
    {
        let Ok(Some(_)) = sizes.size_of(entity) else {
            continue;
        };

        info!(breadboard = ?entity, "Making breadboard visible.");
        *visibility = Visibility::Visible;
    }
}

#[derive(Resource, Deref, DerefMut, Debug, Default)]
pub(super) struct ShowNumbers(bool);

#[derive(SystemParam)]
pub(crate) struct ShowNumbersCheckbox<'w> {
    show: ResMut<'w, ShowNumbers>,
    redraw: ResMut<'w, ForceRedraw>,
}

impl WidgetSystem for ShowNumbersCheckbox<'_> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        _: Self::Args,
    ) -> Self::Output {
        let ShowNumbersCheckbox {
            mut show,
            mut redraw,
        } = state.get_mut(world);

        let mut curr = **show.as_ref();
        if ui.checkbox(&mut curr, "Show Numbers").clicked() {
            **show = curr;

            redraw.set();
        }
    }
}
