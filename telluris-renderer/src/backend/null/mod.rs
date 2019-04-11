use specs::{System, ReadStorage};
use log::*;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct NullRenderer;

impl<'a> System<'a> for NullRenderer {
    type SystemData = ReadStorage<'a, Material>;

    fn run(&mut self, data: Self::SystemData) {
        trace!("NullRenderer.Run");
    }
}
