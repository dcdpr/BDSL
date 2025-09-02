use std::time::Duration;

use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_tweening::{lens::TransformPositionLens, Animator, Tween};

use crate::{plugins::debug::DebugInfiniteZoom, prelude::*};

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct Target(pub Option<Entity>);

impl Target {
    pub fn get(&self) -> Option<Entity> {
        self.0
    }

    pub fn set(&mut self, entity: Entity) {
        self.0 = Some(entity);
    }

    #[expect(dead_code)]
    pub fn reset(&mut self) {
        self.0 = None;
    }
}

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Target>()
            .add_plugins(PanCamPlugin)
            .add_systems(
                Update,
                (
                    setup.run_if(any_with_component::<Camera>.and(run_once)),
                    redraw_during_camera_movement.run_if(camera_moving),
                    focus
                        .run_if(resource_changed::<Target>)
                        .after(AppSet::EntityUpdates),
                ),
            );
    }
}

fn setup(
    mut cmd: Commands,
    camera: Query<Entity, With<Camera>>,
    debug: Option<Res<DebugInfiniteZoom>>,
) {
    let entity = camera.single();
    cmd.entity(entity).insert(PanCam {
        grab_buttons: vec![MouseButton::Left],
        min_scale: debug.map_or(1., |_| 0.1),
        max_scale: 10.,
        ..default()
    });
}

fn focus(
    mut cmd: Commands,
    mut camera: Query<(Entity, &Transform), With<Camera>>,
    target: Res<Target>,
    transforms: Query<&GlobalTransform>,
) {
    let Some(target) = target.get() else { return };
    let (camera_entity, camera_transform) = camera.single_mut();

    let Ok(transform) = transforms.get(target) else {
        warn!(?target, "camera target missing transform");
        return;
    };

    if camera_transform.translation.xy() == transform.translation().xy() {
        return;
    }

    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(750),
        TransformPositionLens {
            start: camera_transform.translation.xyz(),
            end: transform
                .translation()
                .xy()
                .extend(camera_transform.translation.y),
        },
    );

    cmd.entity(camera_entity).insert(Animator::new(tween));
}

fn redraw_during_camera_movement(mut force_redraw: ResMut<ForceRedraw>) {
    force_redraw.set();
}

fn camera_moving(
    camera: Query<&Transform, With<Camera>>,
    target: Res<Target>,
    transforms: Query<&GlobalTransform>,
) -> bool {
    target
        .get()
        .and_then(|target| transforms.get(target).ok())
        .and_then(|target_transform| {
            camera.get_single().ok().map(|camera_transform| {
                camera_transform.translation.xy() != target_transform.translation().xy()
            })
        })
        .unwrap_or_default()
}
