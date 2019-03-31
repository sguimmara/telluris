pub mod vk;

use std::error::Error;
use std::fmt;
use std::sync::Arc;
use crate::objects::{
    handle::Handle,
    texture::Format,
};

pub trait Renderer {
    fn name(&self) -> &str;

    fn allocate_texture_2D(&mut self, width: usize, height: usize, format: Format) -> Handle;

    fn free_texture_2D(&mut self, h: Handle);
}
