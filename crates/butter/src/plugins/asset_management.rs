use bevy::asset::embedded_asset;

use crate::prelude::*;

/// Embed all required assets into the binary.
pub(crate) struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::asset::AssetPlugin::default());

        embedded_asset!(app, "../../assets/fonts/PermanentMarker-Regular.ttf");
        embedded_asset!(app, "../../assets/fonts/ShantellSans-Regular.ttf");
        embedded_asset!(app, "../../assets/textures/arrows.png");
        embedded_asset!(app, "../../assets/textures/lines.png");
    }
}
