//! scene objects

use log::*;

use crate::spatial::transformations::{ecef::ECEF, SpatialReference};
use crate::{DummyModule, Module, Update};

/// The scene is a central object in Telluris.
#[derive(Debug)]
pub struct Scene {
    referential: Box<SpatialReference>,
    modules: Vec<Box<dyn Module>>,
}

impl Scene {
    /// The current spatial reference
    pub fn referential(&self) -> &SpatialReference {
        &*self.referential
    }

    pub fn add_module<T>(&mut self) -> &mut Self
        where T : Module + Default + 'static {
        let m = T::default();
        info!("adding module <{}>", m.name());
        self.modules.push(Box::new(m));
        self
    }
}

impl Default for Scene {
    fn default() -> Self {
        info!("initializing default scene");
        let mut modules = Vec::<Box<Module>>::new();
        modules.push(Box::new(DummyModule {}));
        Scene {
            referential: Box::new(ECEF {}),
            modules,
        }
    }
}

impl Update for Scene {
    fn update(&mut self, dt: f32) {
        for module in &mut self.modules {
            trace!("updating {}", module.name());
            module.update(dt);
        }
    }
}
