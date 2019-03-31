pub mod vk;

pub trait Renderer {
    fn name(&self) -> &str;
}