use bevy::prelude::Component;

pub mod default_fluids;

#[derive(PartialEq, Clone, Debug)]
pub enum MaterialState{
    Solid,
    Liquid,
    Gas
}

#[derive(PartialEq, Clone, Debug)]
///Used to define the properties of a fluid type. Should be instantiated once as a static per fluid type. Said static is then referenced by containers.
pub struct FluidProperties{
    pub state: MaterialState,
    pub molar_mass : f32
}
pub trait FluidComponent : Component + Default{
    const PROPERTIES : FluidProperties;
}


