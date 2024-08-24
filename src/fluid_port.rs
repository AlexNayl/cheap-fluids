use bevy::prelude::*;

///A fluid port is an interface used to connect two fluid containers together, such as a pipe, a valve, or a pump.
///Ports can also connect to a single container, such as a drain, or a fluid spawning source. 
pub trait FluidPort {
    fn get_primary_container (&self) -> Entity;
    fn get_secondary_container (&self) -> Entity;
    fn get_primary_location (&self) -> Vec3;
    fn get_secondary_location (&self) -> Vec3;
    fn get_primary_area (&self) -> f32;
    fn get_secondary_area (&self) -> f32;
}

/// A basic fluid port between two containers , acts like a pipe with no volume
#[derive(Component)]
pub struct BasicFluidPort{
    
}