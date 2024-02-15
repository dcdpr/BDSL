use crate::prelude::*;

use super::{
    place::PlaceCreated,
    shared::{Description, TitleBundle},
    CanvasSet,
};

/// Manage *affordances* in a place.
pub(crate) struct AffordancePlugin;

impl Plugin for AffordancePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AffordanceCreated>().add_systems(
            Update,
            (
                create.run_if(on_event::<PlaceCreated>()),
                create_title.run_if(on_event::<AffordanceCreated>()),
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
#[derive(Bundle, Default)]
struct AffordanceBundle {
    marker: Affordance,
    visibility: VisibilityBundle,
    transform: TransformBundle,
}

#[derive(Event)]
pub(crate) struct AffordanceCreated {
    pub entity: Entity,
    pub name: String,
    pub connections: Vec<ast::Connection>,
}

#[derive(Component)]
struct NestingLevel(usize);

fn create(
    mut cmd: Commands,
    mut places: EventReader<PlaceCreated>,
    mut created: EventWriter<AffordanceCreated>,
) {
    for &PlaceCreated {
        entity,
        ref affordances,
        ..
    } in places.read()
    {
        for ast::Affordance {
            name,
            description,
            connections,
            level,
        } in affordances.clone()
        {
            let span = span!(Level::INFO, "create_affordance", %name, place = ?entity);
            let _enter = span.enter();

            let entity = cmd
                .spawn(AffordanceBundle::default())
                .insert(NestingLevel(level))
                .set_parent(entity)
                .id();

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
        }
    }
}

fn create_title(
    mut cmd: Commands,
    mut places: EventReader<AffordanceCreated>,
    asset_server: Res<AssetServer>,
) {
    for &AffordanceCreated {
        entity, ref name, ..
    } in places.read()
    {
        let span = span!(Level::INFO, "create_affordance_title", %name, place = ?entity);
        let _enter = span.enter();

        let style = TextStyle {
            font_size: 20.,
            color: Color::BLACK,
            font: asset_server.load(
                "embedded://bnb_butter/plugins/../../assets/fonts/PermanentMarker-Regular.ttf",
            ),
        };

        cmd.spawn(TitleBundle::new(name.to_owned()))
            .insert(Text2dBundle {
                text: Text::from_section(name, style).with_alignment(TextAlignment::Center),
                text_anchor: Anchor::TopCenter,
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(200., f32::INFINITY),
                },
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            })
            .set_parent(entity);
    }
}
