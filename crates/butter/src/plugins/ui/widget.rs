pub(super) mod label;
pub(super) mod navbar;

use std::hash::{Hash, Hasher as _};

use bevy_ecs::system::{SystemParam, SystemState};
use bevy_egui::{egui, egui::Context, EguiContext};
use bevy_utils::{AHasher, HashMap};
use bevy_window::PrimaryWindow;

use crate::prelude::*;

pub trait WorldWidgetSystemExt {
    fn root_widget<S: RootWidgetSystem<Args = ()> + 'static>(
        &mut self,
        id: impl Hash,
    ) -> S::Output {
        self.root_widget_with::<S>(id, ())
    }

    fn root_widget_with<S: RootWidgetSystem + 'static>(
        &mut self,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output;

    fn egui_context_scope<R>(&mut self, f: impl FnOnce(&mut Self, Context) -> R) -> R;
}

impl WorldWidgetSystemExt for World {
    fn root_widget_with<S: RootWidgetSystem + 'static>(
        &mut self,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output {
        self.egui_context_scope(|world, mut ctx| {
            let id = WidgetId::new(id);

            if !world.contains_resource::<StateInstances<S>>() {
                let system = std::any::type_name::<S>();
                debug!(system, "Init system state.");

                world.insert_resource(StateInstances::<S> {
                    instances: HashMap::new(),
                });
            }

            world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {
                let cached_state = states.instances.entry(id).or_insert_with(|| {
                    let system = std::any::type_name::<S>();
                    debug!(system, ?id, "Registering system state for root widget.",);

                    SystemState::new(world)
                });

                let output = S::system(world, cached_state, &mut ctx, args);
                cached_state.apply(world);
                output
            })
        })
    }

    fn egui_context_scope<R>(&mut self, f: impl FnOnce(&mut Self, Context) -> R) -> R {
        let ctx = self
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .single_mut(self)
            .get_mut()
            .clone();

        f(self, ctx)
    }
}

pub trait UiWidgetSystemExt {
    fn add_system<S: WidgetSystem<Args = ()> + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
    ) -> S::Output {
        self.add_system_with::<S>(world, id, ())
    }

    fn add_system_with<S: WidgetSystem + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output;
}

impl UiWidgetSystemExt for egui::Ui {
    fn add_system_with<S: WidgetSystem + 'static>(
        &mut self,
        world: &mut World,
        id: impl Hash,
        args: S::Args,
    ) -> S::Output {
        let id = WidgetId::new(id);

        if !world.contains_resource::<StateInstances<S>>() {
            let system = std::any::type_name::<S>();
            debug!(system, "Init system state.");

            world.insert_resource(StateInstances::<S> {
                instances: HashMap::new(),
            });
        }

        world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {
            let cached_state = states.instances.entry(id).or_insert_with(|| {
                let system = std::any::type_name::<S>();
                debug!(system, ?id, "Registering system state for widget.",);

                SystemState::new(world)
            });
            let output = S::system(world, cached_state, self, args);
            cached_state.apply(world);
            output
        })
    }
}

pub trait RootWidgetSystem: SystemParam {
    type Args;
    type Output;

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ctx: &mut egui::Context,
        args: Self::Args,
    ) -> Self::Output;
}

pub trait WidgetSystem: SystemParam {
    type Args;
    type Output;

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        args: Self::Args,
    ) -> Self::Output;
}

#[derive(Resource, Default)]
struct StateInstances<T: SystemParam + 'static> {
    instances: HashMap<WidgetId, SystemState<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);

impl WidgetId {
    pub fn new(id: impl Hash) -> Self {
        let mut hasher = AHasher::default();
        id.hash(&mut hasher);

        WidgetId(hasher.finish())
    }
}
