use std::{any::TypeId, collections::hash_map::Entry};

use bevy::prelude::*;
use std::collections::HashMap;

use crate::fluid::{FluidComponent, FluidProperties};

///A collection of fluids of a homogenous pressure, temperature, and volume.
///Meant to represent the state of fluids inside a vessel such as a room, or a tank, assuming that the fluids 
///are of homogenous temperature. And as they share the same volume they are also of the same pressure.
#[derive(Component)]
pub struct FluidContainer{
    pub volume : f32,
    pub temperature : f32,
    quantities : HashMap<TypeId, f32>,
    total_mol : f32,
    total_mass : f32,
    headless_pressure : f32,
}

impl FluidContainer {
    pub fn new(volume : f32) -> FluidContainer{
        FluidContainer{
            volume: volume,
            temperature:0.,
            quantities:HashMap::new(),
            total_mol:0.,
            total_mass:0.,
            headless_pressure:0.
        }
    }

    pub fn container_processing_system(
        mut containers : Query<(&mut FluidContainer)>,
        constants : Res<crate::PhysicsConstants>
    ){
        // TODO Calculate total mol/mas/pressure of each container
    }

    /// Returns the quantity of the fluid type in moles.
    /// 
    /// Returns 0 if not present in container
    pub fn get_fluid_quantity<T: FluidComponent>(&self) -> f32{
        let type_id = TypeId::of::<T>();
        if let Some(x) = self.quantities.get(&type_id){
            return x.clone();
        }else{
            return 0.0;
        }
    }

    /// Sets quantity of fluid T in given container, adds / removed fluids as necessary
    /// 
    /// if quantity <= 0, fluid will be removed, function will handle marker components for you
    /// 
    /// # Parameters
    /// commands : a valid command queue, will be returned when done
    /// 
    /// entity : the entity of the fluid container component in question
    /// 
    /// quantity : the quantity of the fluid in moles
    /// 
    /// # Returns
    /// Command queue given in parameters for further use.
    pub fn set_fluid_quantity<'a,'b,T: FluidComponent>(
        mut self, 
        mut commands:Commands<'a,'b>, 
        entity: Entity, 
        quantity: f32
    ) ->Commands<'a,'b> {
        // Cases, Adding new fluid, modifying existing, removing existing
        let type_id = TypeId::of::<T>();
        match self.quantities.entry(type_id){
            Entry::Occupied(mut entry)=>{
                if quantity > 0.0{
                    // fluid already exists , set new value
                    entry.insert(quantity);
                    return commands;
                }else{
                    // removing existing fluid and marker component
                    entry.remove_entry();
                    commands.entity(entity).remove::<T>();
                    return commands;
                }
            },
            Entry::Vacant(entry)=>{
                if quantity <= 0.0{
                    // add fluid of quantity 0? skip
                    return commands;
                }else{
                    // add fluid and marker component
                    entry.insert(quantity);
                    commands.entity(entity).insert(T::default());
                    return commands;
                }
            }
        }
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
    /// Aka, headless pressure, meaning pressure without factoring the weight of fluid above the sample point, or pressure in 0g
    pub fn get_basic_pressure(&self) -> f32{
        self.headless_pressure
    }
}


