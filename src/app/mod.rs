use log::*;
use specs::{Builder, Component, ReadStorage, RunNow, System, VecStorage, World};
use winit::{Event, EventsLoop, Window, WindowEvent};
use std::time::Duration;

use telluris_renderer::backend::vk::VkRenderer;
use telluris_renderer::backend::null::NullRenderer;
use telluris_tiler::tiler::{Camera, Tiler};

/// Entry point in a Telluris application
#[derive(Debug)]
pub struct App {}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            trace!("Hello, {:?}", &position);
        }
    }
}

impl App {
    pub fn run(&mut self) {
        info!("entering main loop");
        let mut done = false;
        let mut events_loop = EventsLoop::new();

        let mut world = World::new();
        world.register::<telluris_renderer::material::Material>();
        world.register::<Camera>();

        let window = Window::new(&events_loop).expect("could not create a window");
        // let mut rend = VkRenderer::new(&window).unwrap();
        let mut tiler = Tiler{};
        let mut renderer = NullRenderer{};

        world
            .create_entity()
            .with(Camera{})
            .build();

        let mut dispatcher = specs::DispatcherBuilder::new()
            .with(tiler, "tiler", &[])
            .with_barrier()
            .with(renderer, "renderer", &[])
            .build();

        // let mut hello_world = HelloWorld;

        while !done {
            std::thread::sleep(Duration::from_millis(1000));

            events_loop.poll_events(|ev| match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                _ => (),
            });

            dispatcher.dispatch(&world.res);
            // tiler.run_now(&world.res);
            // rend.run_now(&world.res);
            world.maintain();
        }
    }
}

impl Default for App {
    fn default() -> Self {
        info!("initializing default application");

        App {}
    }
}
