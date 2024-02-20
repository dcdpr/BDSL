use bevy_turborand::RngComponent;

use crate::prelude::*;

#[derive(SystemParam)]
pub(crate) struct Rng<'w, 's> {
    rng: Query<'w, 's, &'static mut RngComponent>,
}

impl Rng<'_, '_> {
    pub fn get(&mut self, entity: Entity) -> RngComponent {
        self.rng.get(entity).cloned().unwrap_or_else(|_| {
            error!(?entity, "RngComponent not found for provided entity.");
            RngComponent::new()
        })
    }
}

/// Any generic Bevy setup requirements that aren't handled by more specific plugins.
pub(crate) struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_turborand::prelude::RngPlugin::default());
    }
}
