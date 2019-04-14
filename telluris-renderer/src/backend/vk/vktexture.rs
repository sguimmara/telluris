use std::sync::Arc;
use vulkano as vk;

#[derive(Clone)]
pub struct VkTexture {
    image: Arc<vk::image::ImageAccess>,
}

impl VkTexture {
    pub fn new(image: Arc<vk::image::ImageAccess>) -> Self {
        VkTexture { image }
    }
}
