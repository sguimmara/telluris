//! Telluris is a free and open source engine specialized in planet rendering.

// pub use telluris_core as core;
// pub use telluris_renderer as renderer;
// pub use telluris_surface as surface;

// use crate::core::scene::Scene;
// use crate::core::Update;

// use log::*;

// // use renderer::{RenderSubmission, Renderer};
// use telluris_core::DummyModule;
// use telluris_surface::surface::Surface;
// use winit::{Event, EventsLoop, Window, WindowBuilder, WindowEvent};

// /// Entry point in a Telluris application
// #[derive(Debug)]
// pub struct App {
//     scene: Box<Scene>,
// }

// impl App {
//     pub fn run(&mut self) -> () {
//         info!("entering main loop");
//         let mut done = false;
//         let mut events_loop = EventsLoop::new();

//         let renderer = Renderer::new(&events_loop);
//         match renderer {
//             Err(r) => {
//                 error!("could not create renderer");
//                 return;
//             }
//             _ => (),
//         }

//         let mut renderer = renderer.unwrap();

//         while !done {
//             std::thread::sleep_ms(100);

//             events_loop.poll_events(|ev| match ev {
//                 Event::WindowEvent {
//                     event: WindowEvent::CloseRequested,
//                     ..
//                 } => done = true,
//                 Event::WindowEvent {
//                     event: WindowEvent::Resized(_),
//                     ..
//                 } => renderer.resize(),
//                 _ => (),
//             });

//             self.scene.update(0.0);

//             // let entities: Vec<_> = Vec::new();
//             // let submission = RenderSubmission {
//             //     entities: &entities,
//             // };
//             // renderer.render(&submission);

//             if done {
//                 break;
//             }
//         }

//         info!("exiting");
//     }

//     pub fn scene(&mut self) -> &mut Scene {
//         &mut *self.scene
//     }
// }

// impl Default for App {
//     fn default() -> Self {
//         info!("initializing default application");

//         let mut app = App {
//             scene: Box::new(Scene::default()),
//         };

//         app.scene()
//             .add_module::<Surface>()
//             .add_module::<DummyModule>();

//         app
//     }
// }
