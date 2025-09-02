use bevy::core::{FrameCountPlugin, TaskPoolPlugin};
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::picking::DefaultPickingPlugins;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;
use bevy::state::app::StatesPlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;

use crate::prelude::*;

/// Any generic Bevy setup requirements that aren't handled by more specific plugins.
pub(crate) struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TaskPoolPlugin::default(),
            RenderPlugin::default(),
            // NOTE: Load this after renderer initialization so that it knows about the supported
            // compressed texture formats
            FrameCountPlugin,
            ImagePlugin::default(),
            CorePipelinePlugin,
            TimePlugin,
            TransformPlugin,
            SpritePlugin::default(),
            TextPlugin,
            StatesPlugin,
            DefaultPickingPlugins,
        ))
        .add_plugins(bevy_tweening::TweeningPlugin);
    }
}
