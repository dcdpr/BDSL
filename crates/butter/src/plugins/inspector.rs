use crate::prelude::*;

/// Generic debugging utilities.
pub(crate) struct InspectorPlugin {
    pub enable: bool,
}

impl Default for InspectorPlugin {
    fn default() -> Self {
        Self { enable: false }
    }
}

impl Plugin for InspectorPlugin {
    #[cfg(feature = "inspector")]
    fn build(&self, app: &mut App) {
        use bevy_input::{common_conditions::input_toggle_active, keyboard::KeyCode};
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        if !self.enable {
            return;
        }

        app.add_plugins((
            bevy_core::TypeRegistrationPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI)),
        ));
    }

    #[cfg(not(feature = "inspector"))]
    fn build(&self, _: &mut App) {
        let _enable = self.enable;
    }
}
