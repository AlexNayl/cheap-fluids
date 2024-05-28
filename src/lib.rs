use bevy::app::Plugin;

pub mod container;
pub mod fluid;
pub mod system;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct FluidPlugin;

impl Plugin for FluidPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}