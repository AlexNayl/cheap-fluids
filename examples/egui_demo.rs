use std::time::Duration;
use bevy::prelude::*;
use bevy_cheap_fluids::fluid_container::FluidContainer;
use bevy_cheap_fluids::fluid::FluidComponent;

fn main(){
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
    );
    app.add_systems(Startup, startup_system);
    app.run();
}

fn startup_system(
    mut commands: Commands
){
    commands.spawn(
        FluidContainer::new(1.)
    );
}

fn oscillator(
    t : Duration,
    period : f32,
    mean : f32,
    amplitude : f32,
) -> f32 {
    amplitude * f32::sin(t.as_secs_f32() * (2.0 * std::f32::consts::PI) / period) + mean
}