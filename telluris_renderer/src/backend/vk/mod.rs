use crate::{
    backend::{Renderer},
    objects::{
        handle::{Handle, HandleType},
        texture::{Format},
    }
};
use log::*;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use vulkano::{
    device::{Device, DeviceExtensions, Queue},
    instance::{Instance, InstanceExtensions, Limits, PhysicalDevice},
    swapchain::Surface,
    image::{
        Dimensions,
        immutable::{ImmutableImage},
    },
};
use vulkano_win::create_vk_surface;
use winit::Window;

pub struct VkRenderer<'a> {
    instance: Arc<Instance>,
    window: &'a Window,
    device: Arc<Device>,
    device_extensions: DeviceExtensions,
    surface: Arc<Surface<&'a Window>>,
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    transfer_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
    next_id: usize,
    textures: HashMap<Handle, usize>,
}

impl<'a> Renderer for VkRenderer<'a> {
    fn name(&self) -> &str {
        "Vulkan"
    }

    fn allocate_texture_2D(&mut self, width: usize, height: usize, format: Format) -> Handle {
        let bytes = format.size() * width * height;
        let h = Handle::texture_2d(self.get_id());
        trace!("allocate {:?} ({:?} bytes)", h, bytes);
        self.textures.insert(h, bytes);

        // ImmutableImage::uninitialized(
        //     self.device.clone(),
        //     dimensions: Dimensions {width, height}, format: F, mipmaps: M, usage: ImageUsage, layout: ImageLayout, queue_families: I)

        h
    }

    fn free_texture_2D(&mut self, h: Handle) {
        assert_eq!(h.ty, HandleType::Texture2D);
        assert!(self.textures.contains_key(&h));
        trace!("free {:?}", h);
        let t = self.textures.remove(&h).unwrap();
        // TODO free vulkan object
    }
}

impl<'a> VkRenderer<'a> {
    fn get_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn new(window: &Window) -> Result<VkRenderer, Box<Error>> {
        info!("initializing");

        let app_info = app_info_from_cargo_toml!();
        let extensions = InstanceExtensions::supported_by_core()?;
        info!("enabled instance extensions: {:#?}", extensions);
        let instance = Instance::new(Some(&app_info), &extensions, None)?;
        let surface = create_vk_surface(window, instance.clone())?;
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

            (
                device,
                ext,
                gfx_queue.clone(),
                comp_queue.clone(),
                xfer_queue.clone(),
                gfx_queue.clone(),
            )
        };
        info!("Device: {}", gpu.name());
        info!("renderer successfully created");

        Ok(VkRenderer {
            instance,
            window,
            device,
            device_extensions,
            surface,
            graphics_queue,
            compute_queue,
            transfer_queue,
            present_queue,
            next_id: 1,
            textures: HashMap::new()
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
}

#[cfg(test)]
mod test {
    #[test]
    fn foo() {
        assert!(Renderer::new().is_ok());
    }
}
