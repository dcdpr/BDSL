use crate::{plugins::file_watcher::FileLoadedEvent, prelude::*};

use super::{Canvas, CanvasSet};

/// Render the breadboard on the window canvas.
pub(super) struct BreadboardPlugin;

impl Plugin for BreadboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BreadboardCreated>().add_systems(
            Update,
            (
                replace.run_if(on_event::<FileLoadedEvent>()),
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

#[derive(Event)]
pub(crate) struct BreadboardCreated {
    pub entity: Entity,
    pub places: Vec<ast::Place>,
}

/// Replace any existing breadboard with the newly loaded file.
#[instrument(skip_all)]
fn replace(
    mut cmd: Commands,
    boards: Query<(Entity, &Name), With<Breadboard>>,
    canvas: Query<Entity, With<Canvas>>,
    mut loaded: EventReader<FileLoadedEvent>,
    mut created: EventWriter<BreadboardCreated>,
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
        created.send(BreadboardCreated { entity, places });
    }
}

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
