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

#[bevy_trait_query::queryable]
pub trait FluidComponent{
    fn get_fluid_properties(&self) -> &'static FluidProperties;
    fn get_quantity(&self) -> &f32;
}


