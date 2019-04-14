pub mod null;
pub mod vk;

use specs::System;

pub trait Renderer<'a>: System<'a> {
    /// Returns the name of the renderer backend.
    fn name(&self) -> &str;

    /// Resize the render target.
    fn resize(&mut self, width: u32, height: u32);
}
