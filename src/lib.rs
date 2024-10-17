use bevy::{app::Plugin, prelude::Resource};

pub mod fluid_container;
pub mod fluid;
pub mod system;
pub mod fluid_port;

pub struct FluidPlugin;

#[derive(Resource)]
pub struct PhysicsConstants{
    molar_gas_constant : f32,
}

impl Plugin for FluidPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(fluid::default_fluids::DefaultFluidPlugin)
        .insert_resource(PhysicsConstants{
            molar_gas_constant : 8.31446261815324
        });
    }
}

