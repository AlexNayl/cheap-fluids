use bevy::app::Plugin;

pub mod container;
pub mod fluid;
pub mod system;

pub struct FluidPlugin;

impl Plugin for FluidPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(fluid::default_fluids::DefaultFluidPlugin);
    }
}