use log::*;
use std::sync::Arc;
use vulkano as vk;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{Surface, Swapchain};
use vulkano_win::create_vk_surface;
use winit::Window;

pub struct Display<'w> {
    device: Arc<vk::device::Device>,
    surface: Arc<Surface<&'w Window>>,
    swapchain: Option<Arc<Swapchain<&'w Window>>>,
    images: Vec<Arc<SwapchainImage<&'w Window>>>,
}

impl<'w> Display<'w> {
    pub fn new(
        instance: &Arc<vk::instance::Instance>,
        device: Arc<vk::device::Device>,
        window: &'w Window,
    ) -> Self {
        let surface = create_vk_surface(window, instance.clone()).unwrap();

        Display {
            device,
            surface,
            swapchain: None,
            images: Vec::new(),
        }
    }

    pub fn recreate_swapchain(&mut self, queue: &vk::device::Queue) {
        let caps = self
            .surface
            .capabilities(self.device.physical_device())
            .unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);

        self.swapchain = None;
        self.images.clear();

        let (swapchain, images) = vk::swapchain::Swapchain::new(
            self.device.clone(),
            self.surface.clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            caps.supported_usage_flags,
            vk::sync::SharingMode::Exclusive(queue.family().id()),
            vk::swapchain::SurfaceTransform::Identity,
            alpha,
            vk::swapchain::PresentMode::Fifo,
            true,
            None,
        )
        .expect("failed to create swapchain");

        trace!("recreated swapchain {:?}", dimensions);

        self.swapchain = Some(swapchain);
        self.images = images;
    }

    pub fn surface(&self) -> Arc<Surface<&'w Window>> {
        self.surface.clone()
    }
}
