//! scene objects

use crate::spatial::transformations::{ecef::ECEF, SpatialReference};
use crate::{DummyModule, Module};

/// The scene is a central object in Telluris.
#[derive(Debug)]
pub struct Scene {
    referential: Box<SpatialReference>,
    //    modules: Box<Module>,
}

impl Scene {
    /// The current spatial reference
    pub fn referential(&self) -> &SpatialReference {
        &*self.referential
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            referential: Box::new(ECEF {}),
        }
    }
}
