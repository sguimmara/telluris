use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Material {}

impl Component for Material {
    type Storage = VecStorage<Self>;
}
