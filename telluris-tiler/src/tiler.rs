use log::*;
use specs;

#[derive(Debug)]
pub struct Camera {}

impl specs::Component for Camera {
    type Storage = specs::HashMapStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Tiler {}

impl<'a> specs::System<'a> for Tiler {
    type SystemData = specs::ReadStorage<'a, Camera>;

    fn run(&mut self, _data: Self::SystemData) {
        trace!("Tiler.run");
    }
}
