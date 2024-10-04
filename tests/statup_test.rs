use bevy_cheap_fluids::*;
use bevy::prelude::*;

///This test should put the plugin through the startup process and immediately exit.
#[test]
fn startup_integration_test(){
    App::new()
    .add_plugins(FluidPlugin)
    .add_systems(Last, exit_system)
    .run();
}

fn exit_system(mut app_exit_events: ResMut<Events<bevy::app::AppExit>>){
    app_exit_events.send(bevy::app::AppExit::Success);
}