//! Rng Plugin: Integrating Random Number Generation
//!
//! Facilitates the use of random number generation within the application by incorporating the
//! [`bevy_turborand`] library. This plugin provides a structured approach to accessing and using
//! [`RngComponent`] across different entities and systems, ensuring consistent and convenient
//! random number generation.

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

pub(crate) struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_turborand::prelude::RngPlugin::default());
    }
}
