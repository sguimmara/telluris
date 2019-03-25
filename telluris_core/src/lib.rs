pub mod components;
pub mod scene;
pub mod spatial;

use std::fmt::Debug;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate nalgebra_glm as glm;

/// Represents an object that can be periodically updated
pub trait Update {
    /// Updates the object, passing the elapsed time, in seconds, since the last update.
    fn update(&mut self, dt: f32) -> ();
}

/// Modules are the basic building blocks in Telluris.
pub trait Module: Debug + Update {
    fn name(&self) -> &'static str;
}

pub type Entity = usize;

/// Component are data storage associated with entities.
pub trait Component: Debug {
    fn name(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct DummyModule {}

impl Module for DummyModule {
    fn name(&self) -> &'static str {
        "dummy"
    }
}

impl Update for DummyModule {
    fn update(&mut self, _dt: f32) {}
}
