use crate::backend::Renderer;
use crate::material::Material;
use log::*;
use specs::{ReadStorage, System};

#[derive(Debug, Clone)]
pub struct NullRenderer;

impl<'a> Renderer<'a> for NullRenderer {
    fn name(&self) -> &str {
        "NullRenderer"
    }

    fn resize(&mut self, width: u32, height: u32) {
        trace!("resize target to {}*{}", width, height);
    }
}

impl<'a> System<'a> for NullRenderer {
    type SystemData = ReadStorage<'a, Material>;

    fn run(&mut self, _data: Self::SystemData) {
        trace!("NullRenderer.Run");
    }
}
