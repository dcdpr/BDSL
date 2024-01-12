use crate::prelude::*;

/// Any generic Bevy setup requirements that aren't handled by more specific plugins.
pub(crate) struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TransformPlugin);
    }
}
