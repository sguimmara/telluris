use telluris_core::{Module, Update};
use log::*;

#[derive(Debug)]
pub struct Surface {

}

impl Surface {
    pub fn new() -> Self {
        Surface {}
    }
}

impl Module for Surface {
    fn name(&self) -> &'static str {
        "surface"
    }
}

impl Update for Surface {
    fn update(&mut self, dt: f32) {
    }
}

impl Default for Surface {
    fn default() -> Self {
        Surface::new()
    }
}