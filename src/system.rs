use std::collections::HashMap;
use thiserror::Error;
use crate::fluid::Fluid;

#[derive(Default)]
pub struct FluidSimSystem{
    ///List of all fluid types 
    fluid_properties : Vec<Fluid>,
    identifier_to_fluid_properties_index : HashMap<String,usize>
}

#[derive(Error, Debug)]
pub enum RegisterFluidError{
    #[error("This fluid identifier has already been registered.")]
    ///Contains the index of the fluid
    FluidAlreadyExists(usize)
}


impl FluidSimSystem{
    ///Returns a vector containing all fluids registered to the system.
    pub fn get_registered_fluids(&self) -> &Vec<Fluid>{
        return &self.fluid_properties;
    }

    ///Returns the fluid registered to the given index.
    pub fn get_fluid(&self, fluid_index : usize) -> Option<&Fluid>{
        return self.fluid_properties.get(fluid_index);
    }

    //Returns the fluid index of a fluid identifier if already registered, otherwise none.
    pub fn get_fluid_index(&self, fluid_identifier : String) -> Option<usize>{
        return Some(self.identifier_to_fluid_properties_index.get(&fluid_identifier)?.clone());
    }

    ///Registers the fluid with the fluid system, allowing it to be used in the fluid system.
    pub fn register_fluid(&mut self, new_fluid: Fluid) -> Result<usize,RegisterFluidError>{
        let target_index = self.fluid_properties.len();


        //Guard statement, if fluid has already been registered, send back an error containing the index of the existing fluid
        if self.identifier_to_fluid_properties_index.contains_key(&new_fluid.identifier){
            let existing_index = self.identifier_to_fluid_properties_index.get(&new_fluid.identifier)
                .expect("Fluid identifier has already been registered, but cant be read, should be impossible.");

            return Err(RegisterFluidError::FluidAlreadyExists(*existing_index));
        }

        self.identifier_to_fluid_properties_index.insert(
            new_fluid.identifier.clone(),
            target_index
        );

        self.fluid_properties.push(new_fluid);
        return Ok(target_index);

    }
}

#[cfg(test)]
mod tests{ 

    use super::*;

    #[test]
    fn fluid_registration_test(){
        let default_fluid = crate::fluid::Fluid::default();
        let mut fluid_system = FluidSimSystem::default();

        let register_result = fluid_system.register_fluid(default_fluid.clone());
        assert!(register_result.is_ok(), "This fluid hasn't been registered yet, should allow to register");
        let fluid_index = register_result.unwrap();

        let duplicate_result = fluid_system.register_fluid(default_fluid.clone());
        if let Err(RegisterFluidError::FluidAlreadyExists(existing_index)) = duplicate_result{
            assert_eq!(fluid_index, existing_index, "On fluid collision error, system should return index of existing fluid.")
        }else{
            panic!("System should not allow entry of duplicate fluids.")
        }

        let get_fluid_result = fluid_system.get_fluid(fluid_index);
        if let Some(fluid_result) = get_fluid_result{
            assert_eq!(fluid_result, &default_fluid, "This was the fluid stored at this index.");
        }else{
            panic!("Should've returned a fluid since this index was registered earlier.");
        }

        let get_fluid_index_result = fluid_system.get_fluid_index(default_fluid.identifier);
        if let Some(index_result) = get_fluid_index_result{
            assert_eq!(index_result, fluid_index, "Fluid index mismatch.");
        }else{
            panic!("This fluid was registered earlier, should've returned.");
        }
    }   

}