use bevy_pancam::{PanCam, PanCamPlugin};

use crate::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default()).add_systems(
            Update,
            setup.run_if(any_with_component::<Camera>.and_then(run_once())),
        );
    }
}

fn setup(mut cmd: Commands, camera: Query<Entity, With<Camera>>) {
    let entity = camera.single();
    cmd.entity(entity).insert(PanCam {
        grab_buttons: vec![MouseButton::Left],
        min_scale: 1.,
        max_scale: Some(10.),
        ..default()
    });
}
