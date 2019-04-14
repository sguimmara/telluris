use log::*;
use specs::{Builder, RunNow, World};
use std::time::Duration;
use winit::{dpi::LogicalSize, Event, EventsLoop, Window, WindowEvent};

use telluris_renderer::backend::vk::VkRenderer;
use telluris_renderer::backend::Renderer;
use telluris_tiler::tiler::{Camera, Tiler};

/// Entry point in a Telluris application
#[derive(Debug)]
pub struct App {}

impl App {
    pub fn run(&mut self) {
        let mut done = false;
        let mut events_loop = EventsLoop::new();

        let mut world = World::new();
        world.register::<telluris_renderer::material::Material>();
        world.register::<Camera>();

        let window = Window::new(&events_loop).expect("could not create a window");
        let mut renderer = VkRenderer::new(&window).unwrap();
        let tiler = Tiler::default();

        world.create_entity().with(Camera {}).build();

        let mut dispatcher = specs::DispatcherBuilder::new()
            .with(tiler, "tiler", &[])
            .with_barrier()
            .build();

        info!("entering main loop");
        while !done {
            std::thread::sleep(Duration::from_millis(50));

            events_loop.poll_events(|ev| match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                Event::WindowEvent {
                    event: WindowEvent::Resized(LogicalSize { width, height }),
                    ..
                } => renderer.resize(width as u32, height as u32),
                _ => (),
            });

            dispatcher.dispatch(&world.res);
            renderer.run_now(&world.res);
            world.maintain();
        }
        info!("exiting main loop");
    }
}

impl Default for App {
    fn default() -> Self {
        info!("initializing default application");

        App {}
    }
}
