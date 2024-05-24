///A collection of fluids of a homogenous pressure, temperature, and volume.
/// 
///Meant to represent the state of fluids inside a vessel such as a room, or a tank, assuming that the fluids are of homogenous temperature. And as they share the same volume they are also of the same pressure.
/// 
pub struct Container{
    volume : f32,
    temperature : f32,
}