use bevy::prelude::*;

use crate::fluid::FluidComponent;

///A collection of fluids of a homogenous pressure, temperature, and volume.
///Meant to represent the state of fluids inside a vessel such as a room, or a tank, assuming that the fluids 
///are of homogenous temperature. And as they share the same volume they are also of the same pressure.
#[derive(Component)]
pub struct FluidContainer{
    pub volume : f32,
    pub temperature : f32,
    total_mol : f32,
    total_mass : f32,
    headless_pressure : f32,
}

impl FluidContainer {
    pub fn new(volume : f32) -> FluidContainer{
        FluidContainer{
            volume: volume,
            temperature:0.,
            total_mol:0.,
            total_mass:0.,
            headless_pressure:0.
        }
    }

    pub fn container_processing_system(
        mut containers : Query<(&mut FluidContainer, &dyn FluidComponent)>,
        constants : Res<crate::ConstantsResource>
    ){
        //This system calculates the headless pressure for each container
        containers.par_iter_mut().for_each(|(mut container, fluids)|{
            // for each entity with a contain in it, sum up all the 
            container.total_mol = 0.0;
            for fluid in fluids.iter(){
                let this_quantity = fluid.get_quantity();
                container.total_mol += this_quantity;
                container.total_mass += this_quantity * fluid.get_fluid_properties().molar_mass;
            }

            container.headless_pressure = (container.total_mol * container.temperature * constants.molar_gas_constant) / container.volume;
        });
    }

    /// Returns the total mass of fluids inside the container as kg.
    pub fn get_fluid_mass(&self) -> f32{
        self.total_mass
    }


    /// Returns the total amount of mols of all fluids inside the container.
    pub fn get_total_mol(&self) -> f32{
        self.total_mol
    }

    /// Returns the basic pressure inside the container
    /// 
    /// Aka, headless pressure, meaning pressure without factoring the weight of fluid above the sample point
    pub fn get_basic_pressure(&self) -> f32{
        self.headless_pressure
    }
}


