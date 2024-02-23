use crate::prelude::*;

use super::{
    place::{Place, PlaceCreated},
    shared::{Body, Description, Header, Index, Title, TitleBundle},
    CanvasSet,
};

/// Manage *affordances* in a place.
pub(super) struct AffordancePlugin;

impl Plugin for AffordancePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AffordanceCreated>().add_systems(
            Update,
            (
                position_affordance.map(err).run_if(
                    |affordances: Query<Entity, With<Affordance>>,
                     titles: Query<&Parent, (With<Title>, Changed<ComputedSize>)>| {
                        titles.iter().any(|parent| affordances.contains(parent.get()))
                    },
                ),
                create.run_if(on_event::<PlaceCreated>()),
                create_title.run_if(on_event::<AffordanceCreated>()),
                // position_affordances,

            )
                .chain()
                .in_set(CanvasSet::Affordance),
        );
    }
}

/// Marker component for place entities.
#[derive(Component, Default)]
struct Affordance;

/// Bundle of required components for place entities.
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

#[derive(Event)]
pub(crate) struct AffordanceCreated {
    pub entity: Entity,
    pub name: String,
    pub connections: Vec<ast::Connection>,
}

#[derive(Component)]
struct NestingLevel(usize);

#[instrument(skip_all)]
fn create(
    mut cmd: Commands,
    mut places: EventReader<PlaceCreated>,
    bodies: Query<(Entity, &Parent), With<Body>>,
    mut created: EventWriter<AffordanceCreated>,
) {
    for &PlaceCreated {
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
                .insert(Padding::default().bottom(10.))
                .set_parent(body)
                .id();

            span.record("affordance", format!("{entity:?}"));

            // Insert description, if one is provided.
            if !description.is_empty() {
                cmd.entity(entity)
                    .insert(Description::from(description.join("\n")));
            }

            created.send(AffordanceCreated {
                entity,
                name,
                connections,
            });

            index += 1;
        }
    }
}

#[instrument(skip_all)]
fn create_title(
    mut cmd: Commands,
    mut places: EventReader<AffordanceCreated>,
    asset_server: Res<AssetServer>,
) {
    for &AffordanceCreated {
        entity, ref name, ..
    } in places.read()
    {
        let span = info_span!("create_affordance_title", %name, affordance = ?entity, title = field::Empty).entered();

        let style = TextStyle {
            font_size: 20.,
            color: Color::BLACK,
            font: asset_server.load(
                "embedded://bnb_butter/plugins/../../assets/fonts/PermanentMarker-Regular.ttf",
            ),
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
