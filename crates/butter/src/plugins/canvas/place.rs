use crate::prelude::*;

use super::{
    breadboard::BreadboardCreated,
    shared::{Description, TitleBundle},
    CanvasSet,
};

/// Manage *places* in a breadboard.
pub(crate) struct PlacePlugin;

impl Plugin for PlacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceCreated>().add_systems(
            Update,
            (
                create.run_if(on_event::<BreadboardCreated>()),
                (create_title, create_underline).run_if(on_event::<PlaceCreated>()),
            )
                .chain()
                .in_set(CanvasSet::Place),
        );
    }
}

/// Marker component for place entities.
#[derive(Component, Default)]
struct Place;

/// Bundle of required components for place entities.
#[derive(Bundle, Default)]
struct PlaceBundle {
    marker: Place,
    visibility: VisibilityBundle,
    transform: TransformBundle,
}

#[derive(Event)]
pub(crate) struct PlaceCreated {
    pub entity: Entity,
    pub name: String,
    pub affordances: Vec<ast::Affordance>,
}

fn create(
    mut cmd: Commands,
    mut breadboard: EventReader<BreadboardCreated>,
    mut created: EventWriter<PlaceCreated>,
) {
    for &BreadboardCreated {
        entity, ref places, ..
    } in breadboard.read()
    {
        for ast::Place {
            name,
            description,
            affordances,
            ..
        } in places.clone()
        {
            let span = span!(Level::INFO, "create_place", breadboard = ?entity);
            let _enter = span.enter();

            let entity = cmd.spawn(PlaceBundle::default()).set_parent(entity).id();

            // Insert description, if one is provided.
            if !description.is_empty() {
                cmd.entity(entity)
                    .insert(Description::from(description.join("\n")));
            }

            created.send(PlaceCreated {
                entity,
                name,
                affordances,
            });
        }
    }
}

fn create_title(
    mut cmd: Commands,
    mut places: EventReader<PlaceCreated>,
    asset_server: Res<AssetServer>,
) {
    for &PlaceCreated {
        entity, ref name, ..
    } in places.read()
    {
        let span = span!(Level::INFO, "create_place_title", %name, place = ?entity);
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

/// Marker component for underline entities.
#[derive(Component, Default)]
pub(crate) struct Underline;

/// Bundle of required components for underline entities.
#[derive(Bundle, Default)]
pub(super) struct UnderlineBundle {
    marker: Underline,
    visibility: VisibilityBundle,
    transform: TransformBundle,
}

fn create_underline(mut cmd: Commands, mut places: EventReader<PlaceCreated>) {
    for &PlaceCreated { entity, .. } in places.read() {
        let span = span!(Level::INFO, "create_place_underline", place = ?entity);
        let _enter = span.enter();

        cmd.spawn(UnderlineBundle::default()).set_parent(entity);
    }
}
