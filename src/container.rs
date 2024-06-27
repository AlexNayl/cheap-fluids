use bevy::prelude::*;

use crate::fluid::FluidComponent;

///A collection of fluids of a homogenous pressure, temperature, and volume.
/// 
///Meant to represent the state of fluids inside a vessel such as a room, or a tank, assuming that the fluids are of homogenous temperature. And as they share the same volume they are also of the same pressure.
/// 
#[derive(Component)]
pub struct Container{
    pub volume : f32,
    pub temperature : f32,
    total_mol : f32,
    headless_pressure : f32,
}

impl Container {
    pub fn container_processing_system(
        mut containers : Query<(&mut Container, &dyn FluidComponent)>,
        constants : Res<crate::ConstantsResource>
    ){
        //This system calculates the headless pressure for each container
        containers.par_iter_mut().for_each(|(mut container, fluids)|{
            // for each entity with a contain in it, sum up all the 
            container.total_mol = 0.0;
            for fluid in fluids.iter(){
                container.total_mol += fluid.get_quantity();
            }

            container.headless_pressure = (container.total_mol * container.temperature * constants.molar_gas_constant) / container.volume;
        });
    }
}


