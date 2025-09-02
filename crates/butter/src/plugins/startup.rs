use crate::prelude::*;

pub(crate) struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[instrument(skip_all)]
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
