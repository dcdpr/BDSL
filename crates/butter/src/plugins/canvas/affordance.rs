//! Affordance Plugin: Facilitating Affordance Entities Within Places
//!
//! Concentrates on the lifecycle and arrangement of *affordances*—interactive or descriptive
//! elements associated with places within a breadboard. The [`AffordancePlugin`] operates by
//! responding to the creation of places and the establishment of affordances, orchestrating their
//! proper positioning and integration within the visual structure of the canvas.
//!
//! For detailed information on individual parts of this plugin, please refer to the respective
//! documentation within this module.

use crate::prelude::*;

use super::{
    place::{Place, PlaceCreatedEvent},
    shared::{Body, Description, Header, Index, Title, TitleBundle},
    CanvasSet,
};

/// Manage *affordances* in a place.
pub(super) struct AffordancePlugin;

impl Plugin for AffordancePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AffordanceCreatedEvent>().add_systems(
            Update,
            (
                position_affordance.map(err).run_if(
                    |affordances: Query<Entity, With<Affordance>>,
                     titles: Query<&Parent, (With<Title>, Changed<ComputedSize>)>| {
                        titles.iter().any(|parent| affordances.contains(parent.get()))
                    },
                ),
                create.run_if(on_event::<PlaceCreatedEvent>()),
                create_title.run_if(on_event::<AffordanceCreatedEvent>()),
                // position_affordances,

            )
                .chain()
                .in_set(CanvasSet::Affordance),
        );
    }
}

/// Marks entities as affordances within places.
///
/// This component is utilized to identify entities that function as affordances in the context of
/// a place. Affordances represent actionable or informational elements within a place.
#[derive(Component, Default)]
struct Affordance;

/// Bundle of required components for affordance entities.
#[derive(Bundle)]
struct AffordanceBundle {
    marker: Affordance,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    size: ComputedSize,
}

impl Default for AffordanceBundle {
    fn default() -> Self {
        Self {
            size: ComputedSize::Inherit,
            marker: default(),
            visibility: default(),
            transform: default(),
        }
    }
}

/// Signals the creation of an affordance entity within a place.
///
/// Dispatched following the successful creation of an affordance, this event carries the new
/// entity's identifier, its name, and a list of connections as defined in the DSL. It facilitates
/// further interactions or behaviors associated with the affordance, enabling systems to respond
/// to its addition and integrate it appropriately within the broader context of the breadboard and
/// its places.
#[derive(Event)]
pub(crate) struct AffordanceCreatedEvent {
    pub entity: Entity,
    pub name: String,
    pub connections: Vec<ast::Connection>,
}

/// Represents the nesting level of an affordance within its place.
///
/// This component quantifies the hierarchical depth of an affordance, indicating how it is nested
/// relative to other affordances within the same place. The nesting level affects visual
/// representation, with indentation or other spatial adjustments used to convey the affordance's
/// position in the hierarchy.
#[derive(Component)]
struct NestingLevel(usize);

/// Spawns affordance entities for each place based on its defined affordances.
///
/// Iterates through [`PlaceCreatedEvent`] instances to generate affordances within the
/// corresponding place's body. Each affordance is created with a specific nesting level and index.
/// Upon successful creation, an [`AffordanceCreatedEvent`] is emitted for each affordance,
/// signaling its readiness for further interaction or processing within the system.
#[instrument(skip_all)]
fn create(
    mut cmd: Commands,
    mut places: EventReader<PlaceCreatedEvent>,
    bodies: Query<(Entity, &Parent), With<Body>>,
    mut created: EventWriter<AffordanceCreatedEvent>,
    tokens: Res<DesignTokens>,
) {
    for &PlaceCreatedEvent {
        entity: place,
        ref affordances,
        ..
    } in places.read()
    {
        let Some(body) = bodies
            .iter()
            .find_map(|(entity, parent)| (parent.get() == place).then_some(entity))
        else {
            warn!(?place, "Place body not found.");
            continue;
        };

        let mut index = 0;
        for ast::Affordance {
            name,
            description,
            connections,
            level,
        } in affordances.clone()
        {
            let span =
                info_span!("create_affordance", %name, ?place, affordance = field::Empty).entered();

            let entity = cmd
                .spawn(AffordanceBundle::default())
                .insert(NestingLevel(level))
                .insert(Index(index))
                .insert(Padding::default().bottom(tokens.canvas.affordance.padding_bottom.as_f32()))
                .set_parent(body)
                .id();

            span.record("affordance", format!("{entity:?}"));

            // Insert description, if one is provided.
            if !description.is_empty() {
                cmd.entity(entity)
                    .insert(Description::from(description.join("\n")));
            }

            created.send(AffordanceCreatedEvent {
                entity,
                name,
                connections,
            });

            index += 1;
        }
    }
}

/// Generates titles for affordance entities based on their creation events.
///
/// For each [`AffordanceCreatedEvent`], this function creates a title entity with specified
/// styling, including font size, color, and alignment.
#[instrument(skip_all)]
fn create_title(
    mut cmd: Commands,
    mut places: EventReader<AffordanceCreatedEvent>,
    asset_server: Res<AssetServer>,
    tokens: Res<DesignTokens>,
) {
    for &AffordanceCreatedEvent {
        entity, ref name, ..
    } in places.read()
    {
        let span = info_span!("create_affordance_title", %name, affordance = ?entity, title = field::Empty).entered();

        let font = &tokens.canvas.affordance.font.primary;
        let style = TextStyle {
            font_size: 16.,
            color: Color::BLACK,
            font: asset_server.load(format!(
                "embedded://bnb_butter/plugins/../../assets/fonts/{font}.ttf"
            )),
        };

        let title = cmd
            .spawn(TitleBundle::new(name.to_owned()))
            .insert(Text2dBundle {
                text: Text::from_section(name, style).with_justify(JustifyText::Center),
                text_anchor: Anchor::TopCenter,
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(200., f32::INFINITY),
                },
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            })
            .set_parent(entity)
            .id();

        span.record("title", format!("{title:?}"));
    }
}

/// Positions affordances within their respective places based on their computed sizes.
///
/// This function aligns affordances vertically within each place, starting directly below the
/// place's header and stacking them according to their index. It calculates the vertical offset
/// for each affordance based on the cumulative height of preceding affordances.
#[instrument(skip_all)]
fn position_affordance(
    places: Query<Entity, With<Place>>,
    headers: Query<(Entity, &Parent), With<Header>>,
    bodies: Query<(Entity, &Parent), With<Body>>,
    sizes: ComputedSizeParam<Without<Transform>>,
    titles: Query<&Parent, (With<Title>, Changed<ComputedSize>)>,
    mut affordances: Query<(Entity, &Parent, &Index, &mut Transform), With<Affordance>>,
) -> Result<(), Error> {
    for place in &places {
        let Some(header_size) = headers
            .iter()
            .find_map(|(header, parent)| (parent.get() == place).then_some(sizes.size_of(header)))
            .transpose()?
            .flatten()
        else {
            debug!(?place, "No place header with known size found.");
            continue;
        };

        let Some(body) = bodies
            .iter()
            .find_map(|(body, parent)| (parent.get() == place).then_some(body))
        else {
            error!(?place, "No place body found.");
            continue;
        };

        let mut affordances: Vec<_> = affordances
            .iter_mut()
            .filter_map(|(affordance, parent, index, transform)| {
                (parent.get() == body).then_some((affordance, index, transform))
            })
            .filter_map(|(affordance, index, transform)| {
                let transform = transform.map_unchanged(|t| &mut t.translation);

                titles
                    .iter()
                    .find_map(|parent| (parent.get() == affordance).then_some((affordance, index)))
                    .map(|(affordance, index)| (affordance, index, transform))
            })
            .collect();

        affordances.sort_by_key(|(_, index, _)| *index);

        let mut affordances_height = header_size.y;
        for (affordance, _, mut translation) in affordances {
            let size = match sizes.size_of(affordance) {
                Ok(Some(size)) => size,
                Ok(None) => continue,
                Err(error) => {
                    error!(%error, "Could not get size of affordance.");
                    continue;
                }
            };

            let height = affordances_height;
            if translation.y != -height {
                translation.y = -height;
            }

            affordances_height += size.y;
        }
    }

    Ok(())
}
