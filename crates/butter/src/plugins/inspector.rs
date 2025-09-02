use crate::prelude::*;

/// Generic debugging utilities.
#[derive(Default)]
pub(crate) struct InspectorPlugin {
    pub enable: bool,
}

impl Plugin for InspectorPlugin {
    #[cfg(feature = "inspector")]
    fn build(&self, app: &mut App) {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        if !self.enable {
            return;
        }

        app.add_plugins((
            bevy::core::TypeRegistrationPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI)),
        ));
    }

    #[cfg(not(feature = "inspector"))]
    fn build(&self, _: &mut App) {
        let _ = self.enable;
    }
}
