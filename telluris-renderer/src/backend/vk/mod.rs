mod display;
mod storage;
mod vktexture;

use crate::texture::texture2d::Texture2d;
use crate::{backend::Renderer, material::Material};
use log::*;
use specs::{ReadStorage, System};
use std::error::Error;
use std::sync::Arc;
use vulkano as vk;
use vulkano::{
    command_buffer::AutoCommandBufferBuilder,
    device::{Device, DeviceExtensions, Queue},
    image::Dimensions,
    instance::{Instance, InstanceExtensions, PhysicalDevice},
    sync,
    sync::GpuFuture,
};
use vulkano_win::VkSurfaceBuild;
use winit::WindowBuilder;

use display::Display;
use storage::Storage;
use vktexture::VkTexture;

#[allow(dead_code)]
pub struct VkRenderer {
    instance: Arc<Instance>,
    device: Arc<Device>,
    device_extensions: DeviceExtensions,
    surface: Arc<vk::swapchain::Surface<winit::Window>>,
    display: Arc<Display>,
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    transfer_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
    storage: Storage,
}

impl<'a> Renderer<'a> for VkRenderer {
    fn name(&self) -> &str {
        "Vulkan"
    }

    fn resize(&mut self, _width: u32, _height: u32) {
        // return; // TODO this breaks because "surface is already in use"
        self.display = Arc::new(self.display.recreate());
    }
}

impl<'a> System<'a> for VkRenderer {
    type SystemData = ReadStorage<'a, Material>;

    fn run(&mut self, _data: Self::SystemData) {
        trace!("VkRenderer.Run");

        let swapchain = self.display.swapchain();

        let (image_num, acquire_future) =
            match vulkano::swapchain::acquire_next_image(swapchain.clone(), None) {
                Ok(r) => r,
                //            Err(AcquireError::OutOfDate) => {
                //                recreate_swapchain = true;
                //                continue;
                //            },
                Err(err) => panic!("{:?}", err),
            };

        let clear_values = vec![[0.1, 0.1, 0.3, 1.0].into()];

        let cmd_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.graphics_queue.family(),
        )
        .unwrap()
        .begin_render_pass(
            self.display.framebuffers()[image_num].clone(),
            false,
            clear_values,
        )
        .unwrap()
        .end_render_pass()
        .unwrap()
        .build()
        .unwrap();

        let mut previous_frame_end = Box::new(sync::now(self.device.clone())) as Box<GpuFuture>;

        let future = previous_frame_end
            .join(acquire_future)
            .then_execute(self.graphics_queue.clone(), cmd_buffer)
            .unwrap()
            .then_swapchain_present(self.graphics_queue.clone(), swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        // match future {
        //     Ok(future) => {
        //         self.frame_future = Box::new(future) as Box<_>;
        //     }
        //     Err(FlushError::OutOfDate) => {
        //         // TODO
        //         self.frame_future = Box::new(sync::now(self.device.clone())) as Box<_>;
        //     }
        //     Err(e) => {
        //         error!("{:?}", e);
        //         self.frame_future = Box::new(sync::now(self.device.clone())) as Box<_>;
        //     }
        // }
    }
}

impl<'a> VkRenderer {
    pub fn new(events_loop: &winit::EventsLoop) -> Result<VkRenderer, Box<Error>> {
        info!("initializing");

        let app_info = app_info_from_cargo_toml!();
        let extensions = InstanceExtensions::supported_by_core()?;
        info!("enabled instance extensions: {:#?}", extensions);
        let instance = Instance::new(Some(&app_info), &extensions, None)?;
        let gpu = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no Vulkan compatible device found");

        let (
            device,
            device_extensions,
            surface,
            graphics_queue,
            compute_queue,
            transfer_queue,
            present_queue,
            display,
        ) = {
            let families = gpu.queue_families();
            let graphics_family = families.clone().find(|&q| q.supports_graphics()).unwrap();
            let compute_family = families.clone().find(|&q| q.supports_compute()).unwrap();
            let xfer_family = families.clone().find(|&q| q.supports_transfers()).unwrap();

            let features = gpu.supported_features();
            let ext = DeviceExtensions::supported_by_device(gpu);
            info!("enabled device extensions: {:#?}", ext);
            info!("enabled device features: {:#?}", features);

            let queue_request = vec![
                (graphics_family, 1.0),
                (compute_family, 0.4),
                (xfer_family, 0.5),
            ];

            let (device, queue_iter) = Device::new(gpu, &features, &ext, queue_request)?;

            let surface = WindowBuilder::new()
                .build_vk_surface(&events_loop, instance.clone())
                .unwrap();

            let queues: Vec<_> = queue_iter.collect();
            let graphics_queue = queues
                .iter()
                .find(|&q| q.family().supports_graphics())
                .unwrap();
            let compute_queue = queues
                .iter()
                .find(|&q| q.family().supports_compute())
                .unwrap();
            let transfer_queue = queues
                .iter()
                .find(|&q| q.family().supports_transfers())
                .unwrap();
            let present_queue = queues
                .iter()
                .find(|&q| surface.is_supported(q.family()).unwrap_or(false))
                .unwrap();

            let display = Display::new(device.clone(), surface.clone(), present_queue);

            (
                device,
                ext,
                surface.clone(),
                graphics_queue.clone(),
                compute_queue.clone(),
                transfer_queue.clone(),
                present_queue.clone(),
                display,
            )
        };
        info!("Device: {}", gpu.name());
        info!("renderer successfully created");

        Ok(VkRenderer {
            instance,
            device,
            device_extensions,
            surface,
            display: Arc::new(display),
            graphics_queue,
            compute_queue,
            transfer_queue,
            present_queue,
            storage: Storage::default(),
        })
    }

    pub fn graphics_queue(&self) -> Arc<Queue> {
        self.graphics_queue.clone()
    }

    pub fn compute_queue(&self) -> Arc<Queue> {
        self.compute_queue.clone()
    }

    pub fn transfer_queue(&self) -> Arc<Queue> {
        self.transfer_queue.clone()
    }

    pub fn present_queue(&self) -> Arc<Queue> {
        self.present_queue.clone()
    }

    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn device_extensions(&self) -> DeviceExtensions {
        self.device_extensions
    }

    pub fn store_texture_2d(&mut self, texture: &Texture2d) {
        let id = texture.id();

        let dims = Dimensions::Dim2d {
            width: texture.width(),
            height: texture.height(),
        };

        let buf = vk::buffer::CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            vk::buffer::BufferUsage::all(),
            texture.data().clone().into_iter(),
        )
        .expect("failed to create buffer");

        let (img, fut) = vk::image::ImmutableImage::from_buffer(
            buf,
            dims,
            vk::format::R8G8B8A8Unorm,
            self.transfer_queue().clone(),
        )
        .unwrap();
        fut.then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let vktexture = VkTexture::new(img);
        self.storage.store_texture_2d(id, Arc::new(vktexture));

        trace!("store {}", texture);
    }
}
