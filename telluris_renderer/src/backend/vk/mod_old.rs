//! The Vulkan renderer

pub mod material;

use log::*;
use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{
    Framebuffer, FramebufferAbstract, RenderPass, RenderPassAbstract, RenderPassDesc, Subpass,
};
use vulkano::image::attachment::*;
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::{PresentMode, Surface, SurfaceTransform, Swapchain};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture, SharingMode};
use vulkano::image::ImageUsage;
use vulkano::image::swapchain::SwapchainImage;

use vulkano_win::VkSurfaceBuild;

use winit::{EventsLoop, Window, WindowBuilder};

/// The Vulkan renderer
pub struct Renderer {
    instance: Arc<Instance>,
    physical_device_index: usize,
    device: Arc<Device>,
    surface: Arc<Surface<Window>>,
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    render_target: Arc<RenderTarget>,
    render_pass: Arc<RenderPassAbstract>,
    frame_future: Box<GpuFuture>,
}

struct RenderTarget {
    pub swapchain: Arc<Swapchain<Window>>,
    pub images: Vec<Arc<SwapchainImage<Window>>>,
//    pub depth_stencil: Arc<AttachmentImage<Format::D24Unorm_S8Uint>>,
    pub framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    device: Arc<Device>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    surface: Arc<Surface<Window>>,
    sharing_mode: SharingMode,
}

/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
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
        .collect::<Vec<_>>()
}

impl RenderTarget {
    pub fn new(
        surface: Arc<Surface<Window>>,
        device: Arc<Device>,
        render_pass: Arc<RenderPassAbstract + Send + Sync>,
        queue: &Queue) -> Self {

        let sharing_mode = SharingMode::Exclusive(queue.family().id());

        let (swapchain, images) = {
            let window = surface.window();
            let caps = surface.capabilities(device.physical_device()).unwrap();
            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;

            let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
                // convert to physical pixels
                let dimensions: (u32, u32) =
                    dimensions.to_physical(window.get_hidpi_factor()).into();
                [dimensions.0, dimensions.1]
            } else {
                error!("could not query window size.");
                panic!("fixme");
            };

            trace!("selected swapchain FIFO mode");

            Swapchain::new(
                device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                initial_dimensions,
                1,
                usage,
                sharing_mode,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                true,
                None,
            ).unwrap()
        };

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        RenderTarget {
            swapchain,
            images,
            framebuffers,
            device,
            render_pass,
            surface,
            sharing_mode
        }
    }

    pub fn recreate(self) -> Self {
        let (swapchain, images) = {
            let window = self.surface.window();
            let caps = self.surface.capabilities(self.device.physical_device()).unwrap();
            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;

            let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
                // convert to physical pixels
                let dimensions: (u32, u32) =
                    dimensions.to_physical(window.get_hidpi_factor()).into();
                [dimensions.0, dimensions.1]
            } else {
                error!("could not query window size.");
                panic!("fixme");
            };

            trace!("selected swapchain FIFO mode");

            Swapchain::new(
                self.device.clone(),
                self.surface.clone(),
                caps.min_image_count,
                format,
                initial_dimensions,
                1,
                usage,
                self.sharing_mode,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                true,
                Some(&self.swapchain),
            ).unwrap()
        };

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };

        let framebuffers =
            window_size_dependent_setup(&images, self.render_pass.clone(), &mut dynamic_state);

        RenderTarget {
            swapchain,
            images,
            framebuffers,
            device: self.device,
            render_pass: self.render_pass,
            surface: self.surface,
            sharing_mode: self.sharing_mode,
        }
    }
}

pub struct Entity {
    material: usize,
    render_queue: usize,
}

pub struct RenderSubmission<'a> {
    pub entities: &'a Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum RendererCreationError {
    Error,
}

impl<'a> Renderer {
    fn default_render_pass(
        device: Arc<Device>,
        color_format: Format,
    ) -> Arc<RenderPassAbstract + Send + Sync> {
        Arc::new(
            single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: color_format,
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
            .unwrap(),
        )
    }

    pub fn new(events_loop: &EventsLoop) -> Result<Renderer, RendererCreationError> {
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

        let graphics_queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
            .unwrap();

        let compute_queue_family = physical
            .queue_families()
            .find(|&q| q.supports_compute())
            .unwrap();

        let mut queues_request: Vec<(QueueFamily, f32)> = Vec::new();
        queues_request.push((graphics_queue_family, 0.5));
        if compute_queue_family.id() != graphics_queue_family.id() {
            queues_request.push((compute_queue_family, 0.5));
        }

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            queues_request.iter().cloned(),
        )
        .unwrap();

        let queues: Vec<_> = queues.collect();

        let gfx_queue = queues
            .iter()
            .find(|&q| q.family().id() == graphics_queue_family.id())
            .unwrap();
        let comp_queue = queues
            .iter()
            .find(|&q| q.family().id() == compute_queue_family.id())
            .unwrap();

        let caps = surface.capabilities(device.physical_device()).unwrap();
        let format = caps.supported_formats[0].0;

        let render_pass = Renderer::default_render_pass(device.clone(), format);
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };

        let previous_frame_end = Box::new(sync::now(device.clone())) as Box<GpuFuture>;

        let target = Arc::new(RenderTarget::new(surface.clone(), device.clone(), render_pass.clone(), gfx_queue));

        let r = Renderer {
            instance,
            physical_device_index: 0,
            device,
            surface,
            graphics_queue: gfx_queue.clone(),
            compute_queue: comp_queue.clone(),
            render_target: target,
            render_pass,
            frame_future: previous_frame_end,
        };

        Ok(r)
    }

    pub fn render(&mut self, submission: &RenderSubmission) {
        trace!(
            "processing submission of {} entities",
            submission.entities.len()
        );

        let (image_num, acquire_future) = match vulkano::swapchain::acquire_next_image(
            self.render_target.swapchain.clone(),
            None,
        ) {
            Ok(r) => r,
            //            Err(AcquireError::OutOfDate) => {
            //                recreate_swapchain = true;
            //                continue;
            //            },
            Err(err) => panic!("{:?}", err),
        };

        let clear_values = vec![[0.2, 0.2, 0.5, 1.0].into()];

//        self.frame_future.cleanup_finished();

        let cmd_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.graphics_queue.family(),
        )
        .unwrap()
        .begin_render_pass(
            self.render_target.framebuffers[image_num].clone(),
            false,
            clear_values,
        )
        .unwrap()

            // TODO put rendering code here

        .end_render_pass()
        .unwrap()
        .build()
        .unwrap();

        let mut previous_frame_end = Box::new(sync::now(self.device.clone())) as Box<GpuFuture>;

        let future = previous_frame_end
            .join(acquire_future)
            .then_execute(self.graphics_queue.clone(), cmd_buffer)
            .unwrap()
            .then_swapchain_present(
                self.graphics_queue.clone(),
                self.render_target.swapchain.clone(),
                image_num,
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.frame_future = Box::new(future) as Box<_>;
            }
            Err(FlushError::OutOfDate) => {
                // TODO
                self.frame_future = Box::new(sync::now(self.device.clone())) as Box<_>;
            }
            Err(e) => {
                error!("{:?}", e);
                self.frame_future = Box::new(sync::now(self.device.clone())) as Box<_>;
            }
        }

//        previous_frame_end.cleanup_finished();
    }

    pub fn resize(&mut self) {
        self.render_target = Arc::new(self.render_target.recreate());
        debug!("resizing renderer");
    }
}
