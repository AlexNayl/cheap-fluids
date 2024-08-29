use bevy::ecs::component::Component;
use bevy::app::Plugin;
use super::*;
pub struct DefaultFluidPlugin;

impl Plugin for DefaultFluidPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn super::FluidComponent, Ice>()
        .register_component_as::<dyn super::FluidComponent,Water>()
        .register_component_as::<dyn super::FluidComponent,Steam>()
        ;
    }
}

//ICE
pub static ICE_PROPERTIES : FluidProperties = FluidProperties{
    state: MaterialState::Solid,
    molar_mass: 0.00180158
};

#[derive(Component)]
pub struct Ice{
    quantity : f32
}

impl FluidComponent for Ice{
    fn get_fluid_properties(&self) -> &'static FluidProperties {
        return &ICE_PROPERTIES;
    }
    
    fn get_quantity(&self) -> &f32 {
        &self.quantity
    }
}


//Water
pub static WATER_PROPERTIES : FluidProperties = FluidProperties{
    state: MaterialState::Liquid,
    molar_mass: 0.00180158
};

#[derive(Component)]
pub struct Water{
    pub quantity : f32
}

impl FluidComponent for Water{
    fn get_fluid_properties(&self) -> &'static FluidProperties {
        return &WATER_PROPERTIES;
    }
    
    fn get_quantity(&self) -> &f32 {
        &self.quantity
    }
}

//Steam
pub static STEAM_PROPERTIES : FluidProperties = FluidProperties{
    state: MaterialState::Gas,
    molar_mass: 0.00180158
};

#[derive(Component)]
pub struct Steam{
    pub quantity : f32
}

impl FluidComponent for Steam{
    fn get_fluid_properties(&self) -> &'static FluidProperties {
        return &STEAM_PROPERTIES;
    }
    
    fn get_quantity(&self) -> &f32 {
        &self.quantity
    }
}


