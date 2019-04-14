use super::vktexture::VkTexture;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct Storage {
    textures: HashMap<usize, Arc<VkTexture>>,
}

impl Storage {
    pub fn store_texture_2d(&mut self, id: usize, texture: Arc<VkTexture>) {
        self.textures.insert(id, texture);
    }
}
