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
    device::{Device, DeviceExtensions, Queue},
    image::Dimensions,
    instance::{Instance, InstanceExtensions, PhysicalDevice},
    sync::GpuFuture,
};

use display::Display;
use storage::Storage;
use vktexture::VkTexture;

#[allow(dead_code)]
pub struct VkRenderer<'a> {
    instance: Arc<Instance>,
    device: Arc<Device>,
    device_extensions: DeviceExtensions,
    display: Display<'a>,
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    transfer_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
    storage: Storage,
}

impl<'a> Renderer<'a> for VkRenderer<'a> {
    fn name(&self) -> &str {
        "Vulkan"
    }

    fn resize(&mut self, _width: u32, _height: u32) {
        self.display.recreate_swapchain(&self.present_queue);
    }
}

impl<'a, 'w> System<'a> for VkRenderer<'w> {
    type SystemData = ReadStorage<'a, Material>;

    fn run(&mut self, _data: Self::SystemData) {
        trace!("VkRenderer.Run");
    }
}

impl<'a> VkRenderer<'a> {
    pub fn new(window: &winit::Window) -> Result<VkRenderer, Box<Error>> {
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

            let mut display = Display::new(&instance, device.clone(), window);

            let queues: Vec<_> = queue_iter.collect();
            let gfx_queue = queues
                .iter()
                .find(|&q| q.family().supports_graphics())
                .unwrap();
            let comp_queue = queues
                .iter()
                .find(|&q| q.family().supports_compute())
                .unwrap();
            let xfer_queue = queues
                .iter()
                .find(|&q| q.family().supports_transfers())
                .unwrap();
            let prs_queue = queues
                .iter()
                .find(|&q| display.surface().is_supported(q.family()).unwrap_or(false))
                .unwrap();

            display.recreate_swapchain(prs_queue);

            (
                device,
                ext,
                gfx_queue.clone(),
                comp_queue.clone(),
                xfer_queue.clone(),
                prs_queue.clone(),
                display,
            )
        };
        info!("Device: {}", gpu.name());
        info!("renderer successfully created");

        Ok(VkRenderer {
            instance,
            device,
            device_extensions,
            display,
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
