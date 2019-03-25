pub mod material;

use log::*;
use std::sync::Arc;

use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::swapchain::{Swapchain, Surface, PresentMode, SurfaceTransform};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::framebuffer::{Framebuffer, RenderPassDesc, RenderPass, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::format::{Format};
use vulkano::pipeline::viewport::Viewport;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;

use vulkano::image::swapchain::{SwapchainImage};

use vulkano_win::VkSurfaceBuild;

use winit::{EventsLoop, Window, WindowBuilder};

pub struct Renderer {
    instance: Arc<Instance>,
    physical_device_index: usize,
    device: Arc<Device>,
    surface: Arc<Surface<Window>>,
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    render_target: Arc<RenderTarget>,
    render_pass: Arc<RenderPassAbstract>
}

struct RenderTarget {
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
}

pub struct Entity {
    material: usize,
    render_queue: usize
}

pub struct RenderSubmission<'a> {
    entities: &'a Vec<Entity>
}

#[derive(Debug, Clone)]
pub enum RendererCreationError { 
    Error
}

impl<'a> Renderer {
    fn default_render_pass(device: Arc<Device>, color_format: Format) -> Arc<RenderPassAbstract + Send + Sync> {
        Arc::new(single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        ).unwrap())
    }

    pub fn new(events_loop: EventsLoop) -> Result<Renderer, RendererCreationError> {
        info!("initializing renderer");

        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("could not create Vulkan instance")
        };

        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("could not select physical device");
        info!("selected device {} ({:?})", physical.name(), physical.ty());

        let surface = WindowBuilder::new()
            .build_vk_surface(&events_loop, instance.clone())
            .expect("could not create Vulkan surface");

        let graphics_queue_family = physical.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).unwrap();

        let compute_queue_family = physical.queue_families().find(|&q| {
            q.supports_compute()
        }).unwrap();
 
        let mut queues_request : Vec<(QueueFamily, f32)> = Vec::new();
        queues_request.push((graphics_queue_family, 0.5));
        if compute_queue_family.id() != graphics_queue_family.id() {
            queues_request.push((compute_queue_family, 0.5));
        }

        let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
        let (device, queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            queues_request.iter().cloned()).unwrap();

        let queues : Vec<_> = queues.collect();

        let gfx_queue = queues.iter().find(|&q| q.family().id() == graphics_queue_family.id()).unwrap();
        let comp_queue = queues.iter().find(|&q| q.family().id() == compute_queue_family.id()).unwrap();

        let (swapchain, images) = {
            let window = surface.window();
            let caps = surface.capabilities(physical).unwrap();
            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;

            let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
                // convert to physical pixels
                let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
                [dimensions.0, dimensions.1]
            } else {
                error!("could not query window size.");
                panic!("fixme");
            };

            trace!("selected swapchain FIFO mode");

            Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                initial_dimensions, 1, usage, &queues[0], SurfaceTransform::Identity, alpha,
                PresentMode::Fifo, true, None).unwrap()
        };

        let render_pass = Renderer::default_render_pass(device.clone(), swapchain.format());

        let r = Renderer {
            instance,
            physical_device_index: 0,
            device,
            surface,
            graphics_queue: gfx_queue.clone(),
            compute_queue: comp_queue.clone(),
            render_target: Arc::new(RenderTarget{swapchain, images}),
            render_pass
        };

        Ok(r)
    }

    pub fn render(&self, submission: &RenderSubmission) {
        trace!("processing submission of {} entities", submission.entities.len());
    }
}
