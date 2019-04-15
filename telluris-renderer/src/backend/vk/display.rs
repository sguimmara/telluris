use log::*;
use std::sync::Arc;
use vulkano as vk;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::framebuffer::{Framebuffer, RenderPass, FramebufferAbstract, RenderPassAbstract, Subpass, RenderPassDesc};
use vulkano::pipeline::viewport::Viewport;
use vulkano_win::VkSurfaceBuild;
use winit::{Window, WindowBuilder, EventsLoop};

pub struct Display {
    device: Arc<vk::device::Device>,
    swapchain: Arc<Swapchain<Window>>,
    render_pass: Arc<RenderPassAbstract>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
}

impl Display {
    pub fn new(
        device: Arc<vk::device::Device>,
        surface: Arc<Surface<Window>>,
        present_queue: &vk::device::Queue
    ) -> Self {
        let caps = surface
            .capabilities(device.physical_device())
            .unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);

        let (swapchain, images) = vk::swapchain::Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            caps.supported_usage_flags,
            vk::sync::SharingMode::Exclusive(present_queue.family().id()),
            vk::swapchain::SurfaceTransform::Identity,
            alpha,
            vk::swapchain::PresentMode::Fifo,
            true,
            None,
        )
        .expect("failed to create swapchain");

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        };

        let render_pass = Arc::new(
            single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: format,
                        samples: 1,
                    }/*,
                    depth: {
                        load: Clear,
                        store: DontCare,
                        format: Format::D16Unorm,
                        samples: 1,
                    }*/
                },
                pass: {
                    color: [color],
                    depth_stencil: {/*depth*/}
                }
            )
            .unwrap());

        let framebuffers = images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<FramebufferAbstract + Send + Sync>
            })
        .collect::<Vec<_>>();

        trace!("recreated swapchain {:?}", dimensions);

        Display {
            device,
            swapchain,
            render_pass,
            images,
            framebuffers,
        }
    }

    pub fn swapchain(&self) -> Arc<Swapchain<Window>> {
        self.swapchain.clone()
    }

    pub fn framebuffers(&self) -> &Vec<Arc<FramebufferAbstract + Send + Sync>> {
        &self.framebuffers
    }
}
