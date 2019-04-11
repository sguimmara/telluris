pub mod vk;
pub mod null;

use specs::System;
// use crate::objects::{handle::Handle, texture::Format};

pub trait Renderer<'a> : System<'a> {
    fn name(&self) -> &str;

    // fn allocate_texture_2d(&mut self, width: usize, height: usize, format: Format) -> Handle;

    // fn free_texture_2d(&mut self, h: Handle);
}
