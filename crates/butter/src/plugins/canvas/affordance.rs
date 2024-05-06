//! Affordance Plugin: Facilitating Affordance Entities Within Places
//!
//! Concentrates on the lifecycle and arrangement of *affordances*—interactive or descriptive
//! elements associated with places within a breadboard. The [`AffordancePlugin`] operates by
//! responding to the creation of places and the establishment of affordances, orchestrating their
//! proper positioning and integration within the visual structure of the canvas.
//!
//! For detailed information on individual parts of this plugin, please refer to the respective
//! documentation within this module.

use bevy_utils::HashMap;

use crate::prelude::*;

use super::{
    breadboard::ShowNumbers,
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
                (
                    position_affordance.map(err).run_if(run_position_affordance),
                    create.run_if(on_event::<PlaceCreatedEvent>()),
                )
                    .chain(),
                toggle_numbering.run_if(resource_changed::<ShowNumbers>),
            )
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
    indices: Query<&Index>,
    bodies: Query<(Entity, &Parent), With<Body>>,
    mut created: EventWriter<AffordanceCreatedEvent>,
    asset_server: Res<AssetServer>,
    tokens: Res<DesignTokens>,
) {
    for &PlaceCreatedEvent {
        entity: place,
        ref affordances,
        ..
    } in places.read()
    {
        let place_index = **indices.get(place).expect("index exists");

        let Some(body) = bodies
            .iter()
            .find_map(|(entity, parent)| (parent.get() == place).then_some(entity))
        else {
            warn!(?place, "Place body not found.");
            continue;
        };

        // TODO:
        //
        // Created nested affordances based on affordance level (e.g. a level 1 affordance
        // following a level 0 affordance will become a child of the level 0 affordance).
        //
        // This makes it easier to render, and reposition a group of nested affordances. It also
        // makes it easier to hide a tree of nested affordances by iterating all children and
        // setting them as invisible.
        let mut index = 0;
        let mut indices = HashMap::new();
        for ast::Affordance {
            name,
            description,
            connections,
            level,
        } in affordances.clone()
        {
            indices.entry(level).or_default();

            let span =
                info_span!("create_affordance", %name, ?place, affordance = field::Empty).entered();

            let affordance = cmd
                .spawn(AffordanceBundle::default())
                .insert(NestingLevel(level))
                .insert(Index(index))
                .insert(Padding::default().bottom(tokens.canvas.affordance.padding_bottom.as_f32()))
                .set_parent(body)
                .id();

            span.record("affordance", format!("{affordance:?}"));

            // Insert description, if one is provided.
            if !description.is_empty() {
                cmd.entity(affordance)
                    .insert(Description::from(description.join("\n")));
            }

            let font_family = &tokens.canvas.affordance.font.primary;
            let font = asset_server.load(format!(
                "embedded://bnb_butter/plugins/../../assets/fonts/{font_family}.ttf"
            ));

            let title = create_title(&mut cmd, place_index, &indices, level, &name, font, &tokens);
            cmd.entity(affordance).add_child(title);

            created.send(AffordanceCreatedEvent {
                entity: affordance,
                name,
                connections,
            });

            *indices.get_mut(&level).unwrap() += 1;
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
    cmd: &mut Commands,
    place_index: usize,
    indices: &HashMap<usize, usize>,
    level: usize,
    name: &str,
    font: Handle<Font>,
    tokens: &DesignTokens,
) -> Entity {
    let span = info_span!("spawn", %name, title = field::Empty).entered();

    let name_style = TextStyle {
        font_size: 16.,
        color: Color::BLACK,
        font: font.clone(),
    };

    let number_style = TextStyle {
        font_size: 13.,
        color: Color::DARK_GRAY,
        font,
    };

    let x = tokens.canvas.affordance.level_padding.as_f32() * level as f32;

    let mut numbers = format!("{}.", place_index + 1);
    for lvl in 0..=level {
        let index = indices.get(&lvl).copied().unwrap_or_default() + 1;
        let index = if lvl == level {
            index
        } else {
            index.saturating_sub(1)
        };

        numbers.push_str(&format!("{}.", index))
    }
    numbers.push(' ');

    let title = cmd
        .spawn(TitleBundle::new(name.to_owned()))
        .insert(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(numbers, number_style),
                TextSection::new(name, name_style),
            ]),
            // TODO: left-align text, based on the left edge of the place (title).
            text_anchor: Anchor::TopLeft,
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(200., f32::INFINITY),
            },
            transform: Transform::from_xyz(-40. + x, 0., 2.),
            ..default()
        })
        .id();

    span.record("title", format!("{title:?}"));

    title
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

fn run_position_affordance(
    affordances: Query<Entity, With<Affordance>>,
    titles: Query<&Parent, (With<Title>, Changed<ComputedSize>)>,
) -> bool {
    titles
        .iter()
        .any(|parent| affordances.contains(parent.get()))
}

fn toggle_numbering(
    show: Res<ShowNumbers>,
    mut titles: Query<(&Parent, &mut Text), With<Title>>,
    affordances: Query<Entity, With<Affordance>>,
    places: Query<Entity, With<Place>>,
    indices: Query<&Index>,
    levels: Query<&NestingLevel>,
    parents: Query<&Parent>,
) {
    let texts = titles.iter_mut().filter_map(|(parent, text)| {
        affordances.get(parent.get()).ok().and_then(|affordance| {
            levels.get(affordance).ok().and_then(|level| {
                parents
                    .iter_ancestors(affordance)
                    .find_map(|v| places.get(v).and_then(|place| indices.get(place)).ok())
                    .map(|place_index| (place_index, level, text))
            })
        })
    });

    let mut place_indices: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for (&Index(place_index), &NestingLevel(level), mut text) in texts {
        if **show {
            let indices = place_indices.entry(place_index).or_default();
            indices.entry(level).or_default();

            let mut numbers = format!("{}.", place_index + 1);

            for lvl in 0..=level {
                let index: usize = indices.get(&lvl).copied().unwrap_or_default() + 1;
                let index = if lvl == level {
                    index
                } else {
                    index.saturating_sub(1)
                };

                numbers.push_str(&format!("{}.", index))
            }
            numbers.push(' ');

            text.sections[0].value = numbers;

            *indices.get_mut(&level).unwrap() += 1;
        } else {
            text.sections[0].value.clear();
        }
    }
}
