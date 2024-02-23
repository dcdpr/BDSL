//! Place Plugin: Orchestrating Place Entities Within Breadboards
//!
//! Dedicated to the management and organization of *places* within a breadboard, the
//! [`PlacePlugin`] integrates closely with the broader canvas rendering system. It is designed to
//! respond dynamically to the creation of breadboards by initializing and managing place entities,
//! which represent distinct functional areas or components within a breadboard's layout.
//!
//! The plugin employs a series of event-driven and conditional systems to adapt the layout and
//! presentation of places in real-time, enhancing the application's interactivity and user experience.
//!
//! For detailed information on individual parts of this plugin, please refer to the respective
//! documentation within this module.

use bevy_asset::Assets;
use bevy_internal::hierarchy::Parent;
use bevy_sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout};
use tracing::field;

use crate::prelude::*;

use super::{
    breadboard::BreadboardCreatedEvent,
    shared::{Body, BodyBundle, Description, Header, HeaderBundle, Title, TitleBundle},
    CanvasSet,
};

/// Facilitates the management of *places* within breadboards.
///
/// For a detailed overview of the plugin's architecture and functionalities, refer to the
/// module-level documentation.
pub(super) struct PlacePlugin;

impl Plugin for PlacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceCreatedEvent>().add_systems(
            Update,
            (
                create.run_if(on_event::<BreadboardCreatedEvent>()),
                position_underline.run_if(
                    |underlines: Query<&Parent, With<Underline>>,
                     titles: Query<&Parent, Changed<ComputedSize>>,
                     headers: Query<(), With<Header>>| {
                        // Get all underlines.
                        underlines.iter().any(|v| {
                            titles
                                // And for those underlines, find the parent title with changed
                                // computed size.
                                .get(v.get())
                                // And those titles should be part of the place `Header`.
                                .map(|v| headers.contains(v.get()))
                                // Check if there's a match, and skip positioning the underline if
                                // there isn't.
                                .is_ok()
                        })
                    },
                ),
                position_body.run_if(
                    |bodies: Query<&Parent, With<Body>>,
                     headers: Query<&Parent, (With<Header>, Changed<ComputedSize>)>| {
                        bodies.iter().any(|b| {
                            headers.iter().any(|h| h.get() == b.get())
                        })
                    },
                ),
            )
                .chain()
                .in_set(CanvasSet::Place),
        );
    }
}

/// Denotes entities as individual places within a breadboard.
///
/// Applied to entities to mark them as places, which are conceptual areas or components within a
/// breadboard's structure. This marker is essential for distinguishing these entities within the
/// ECS architecture, facilitating targeted queries and operations on places.
#[derive(Component, Default)]
pub(super) struct Place;

/// Bundle of required components for place entities.
#[derive(Bundle)]
struct PlaceBundle {
    marker: Place,
    visibility: VisibilityBundle,
    transform: TransformBundle,
    size: ComputedSize,
}

impl Default for PlaceBundle {
    fn default() -> Self {
        Self {
            size: ComputedSize::Inherit,
            marker: default(),
            visibility: default(),
            transform: default(),
        }
    }
}

/// Signifies the creation of a place entity within the breadboard.
///
/// Dispatched upon the successful creation of a place entity, this event carries the entity's
/// identifier and a list of its affordances as defined in the breadboard's DSL. It enables other
/// systems and components to react to the addition of new places, facilitating further
/// initialization or modification of affordances associated with the place.
#[derive(Event)]
pub(crate) struct PlaceCreatedEvent {
    pub entity: Entity,
    pub affordances: Vec<ast::Affordance>,
}

/// Initiates place entities within a newly created breadboard.
///
/// Iterates over [`BreadboardCreatedEvent`]s to spawn place entities as defined in the event's
/// associated breadboard. Each place entity is then structured with a header and body, and
/// potentially a description, reflecting its definition in the DSL. This process includes
/// generating unique visual elements for each place, such as titles and underlines, utilizing a
/// seeded random number generator for consistency. Upon successful creation, a
/// [`PlaceCreatedEvent`] is emitted for each place, indicating its readiness for further
/// interactions or modifications.
#[instrument(skip_all)]
fn create(
    mut cmd: Commands,
    mut breadboard: EventReader<BreadboardCreatedEvent>,
    mut created: EventWriter<PlaceCreatedEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut rng: Rng,
) {
    for &BreadboardCreatedEvent {
        entity: breadboard,
        ref places,
        ..
    } in breadboard.read()
    {
        let mut rng = rng.get(breadboard);

        for ast::Place {
            name,
            description,
            affordances,
            ..
        } in places.clone()
        {
            let span = info_span!("spawn", ?breadboard, place = field::Empty).entered();

            let place = cmd
                .spawn(PlaceBundle::default())
                .set_parent(breadboard)
                .id();
            span.record("place", format!("{place:?}"));

            // Insert description, if one is provided.
            if !description.is_empty() {
                cmd.entity(place)
                    .insert(Description::from(description.join("\n")));
            }

            let header = create_header(
                &mut cmd,
                name,
                &asset_server,
                &mut texture_atlases,
                &mut rng,
            );
            cmd.entity(place).add_child(header);

            let body = create_body(&mut cmd);
            cmd.entity(place).add_child(body);

            // TODO: Should this trigger *after* title & underline are positioned?
            created.send(PlaceCreatedEvent {
                entity: place,
                affordances,
            });
        }
    }
}

/// Constructs a header entity for a place, including a title and an underline.
///
/// Utilizes the name provided to generate a visually distinct header for each place. This includes
/// loading specific font and texture assets to create a title entity and an underline entity,
/// respectively. The underline is positioned relative to the title, and both are grouped under a
/// single header entity with custom padding at the bottom.
#[instrument(skip_all)]
fn create_header(
    cmd: &mut Commands,
    name: String,
    asset_server: &AssetServer,
    atlasses: &mut Assets<TextureAtlasLayout>,
    rng: &mut RngComponent,
) -> Entity {
    let span = info_span!("spawn", %name, header = field::Empty).entered();

    let font = asset_server
        .load("embedded://bnb_butter/plugins/../../assets/fonts/PermanentMarker-Regular.ttf");
    let image = asset_server.load("embedded://bnb_butter/plugins/../../assets/textures/lines.png");

    let title = create_title(cmd, &name, font);
    let underline = create_underline(cmd, atlasses, image, rng);
    cmd.entity(title).add_child(underline);

    let header = cmd
        .spawn(HeaderBundle::default())
        .insert(Padding::default().bottom(20.))
        .add_child(title)
        .id();
    span.record("header", format!("{header:?}"));

    header
}

/// Creates a title entity for a place with specified styling.
///
/// Generates a title entity using the provided name and font, applying a defined [`TextStyle`] to
/// ensure consistent visual appearance. The title is centered both horizontally and vertically,
/// with specific bounds to accommodate the text size.
#[instrument(skip_all)]
fn create_title(cmd: &mut Commands, name: &str, font: Handle<Font>) -> Entity {
    let style = TextStyle {
        font_size: 20.,
        color: Color::BLACK,
        font,
    };

    cmd.spawn(TitleBundle::new(name.to_owned()))
        .insert(Text2dBundle {
            text: Text::from_section(name, style).with_justify(JustifyText::Center),
            text_anchor: Anchor::TopCenter,
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(200., f32::INFINITY),
            },
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        })
        .id()
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

/// Generates an underline entity with randomized visual attributes.
///
/// Creates an underline entity for a title, employing a texture atlas to allow for varied visual
/// styles. The underline's size and orientation are randomly determined, providing a unique,
/// hand-drawn feel to each instance. This variability is achieved through a combination of size,
/// position, rotation, and texture selection, with randomness seeded to ensure consistent
/// presentation across sessions.
#[instrument(skip_all)]
fn create_underline(
    cmd: &mut Commands,
    atlasses: &mut Assets<TextureAtlasLayout>,
    texture: Handle<Image>,
    rng: &mut RngComponent,
) -> Entity {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(1420.0, 80.0), 1, 20, None, None);
    let layout = atlasses.add(layout);

    // TODO
    // let range = match theme.as_ref() {
    //     Theme::Light => 0..10,
    //     Theme::Dark => 10..20,
    // };
    let range = 0..10;

    let custom_size = Vec2::new(rng.usize(130..220) as f32, rng.usize(8..12) as f32);
    let transform = Transform {
        translation: Vec3::new(0., rng.isize(-6..2) as f32, 1.9),
        rotation: Quat::from_rotation_z((rng.isize(-2..2) / 100) as f32),
        ..default()
    };

    let underline = cmd
        .spawn(UnderlineBundle::default())
        .insert(SpriteSheetBundle {
            atlas: TextureAtlas {
                index: rng.usize(range),
                layout,
            },
            sprite: bevy_sprite::Sprite {
                custom_size: Some(custom_size),
                flip_x: rng.bool(),
                flip_y: rng.bool(),
                ..default()
            },
            transform,
            texture,
            ..default()
        })
        .insert(ComputedSize::Static(custom_size))
        .id();

    underline
}

/// Adjusts underline positions relative to their associated titles.
///
/// This system repositions underlines directly beneath the titles of places, ensuring visual
/// alignment and consistency. It calculates the new position based on the computed size of the
/// title, effectively moving the underline to sit neatly below the title text.
#[instrument(skip_all)]
fn position_underline(
    headers: Query<(), With<Header>>,
    titles: Query<(Entity, &Parent), With<Title>>,
    sizes: ComputedSizeParam<Without<Underline>>,
    mut transforms: Query<(Entity, &Parent, &mut Transform), With<Underline>>,
) {
    transforms
        .iter_mut()
        .filter_map(|(underline, parent, transform)| {
            let transform = transform.map_unchanged(|t| &mut t.translation);

            titles
                .get(parent.get())
                .and_then(|(title, parent)| {
                    headers
                        .get(parent.get())
                        .map(|_| (underline, transform, sizes.size_of(title)))
                })
                .ok()
        })
        .for_each(|(underline, mut translation, size)| match size {
            Ok(Some(size)) => {
                translation.y = -size.y;
                info!(
                    ?underline,
                    ?translation,
                    "Repositioned place title underline."
                );
            }
            Ok(None) => {
                debug!(?underline, "Waiting on pending size.")
            }
            Err(error) => error!(?underline, %error, "Unexpected error."),
        });
}

/// Creates a body entity for a place.
///
/// Initiates a body entity with default settings, serving as a container for additional components
/// or information related to the place.
#[instrument(skip_all)]
fn create_body(cmd: &mut Commands) -> Entity {
    let span = info_span!("spawn", body = field::Empty).entered();

    let body = cmd.spawn(BodyBundle::default()).id();
    span.record("body", format!("{body:?}"));

    body
}

/// Adjusts the position of body entities relative to their corresponding headers.
///
/// This system updates the position of body entities to align with the bottom edge of their
/// associated header entities, based on the header's computed size.
#[instrument(skip_all)]
fn position_body(
    headers: Query<(Entity, &Parent), (With<Header>, Changed<ComputedSize>)>,
    sizes: ComputedSizeParam<Without<Body>>,
    mut transforms: Query<(Entity, &Parent, &mut Transform), With<Body>>,
) {
    transforms
        .iter_mut()
        .filter_map(|(body, parent, transform)| {
            let transform = transform.map_unchanged(|t| &mut t.translation);

            headers
                .iter()
                .find_map(|(header, p)| {
                    (p.get() == parent.get()).then_some((body, sizes.size_of(header)))
                })
                .map(|(body, size)| (body, transform, size))
        })
        .for_each(|(body, mut translation, size)| match size {
            Ok(Some(size)) => {
                translation.y = -size.y;
                info!(?body, ?translation, "Repositioned place body.");
            }
            Ok(None) => {
                debug!(?body, "Waiting on pending size.")
            }
            Err(error) => error!(?body, %error, "Unexpected error."),
        });
}
