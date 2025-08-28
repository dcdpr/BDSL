mod camera;

use crate::prelude::*;

pub(crate) use camera::Target;

/// Handle any input in the app.
pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::input::InputPlugin)
            .add_plugins(camera::CameraPlugin);
    }
}
