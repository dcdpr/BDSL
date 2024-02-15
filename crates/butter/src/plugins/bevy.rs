use bevy_core::{FrameCountPlugin, TaskPoolPlugin};
use bevy_core_pipeline::CorePipelinePlugin;
use bevy_render::RenderPlugin;
use bevy_sprite::SpritePlugin;
use bevy_text::TextPlugin;
use bevy_time::TimePlugin;

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
            SpritePlugin,
            TextPlugin,
        ));
    }
}
