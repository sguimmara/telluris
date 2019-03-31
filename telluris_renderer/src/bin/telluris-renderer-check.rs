use std::sync::{Arc, Mutex};
use log::*;
use simplelog::*;
use std::fs::File;
use telluris_renderer::
{
    backend::vk::VkRenderer,
    objects::texture::{Texture2D, Format}
};

use winit::*;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("telluris-renderer-check.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("checking graphics configuration...");


    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).expect("could create a window");
    let rend = VkRenderer::new(&window).unwrap();
    let mutex = Arc::new(Mutex::new(rend));

    for i in 0..1000 {
        let tex = Texture2D::new(mutex.clone(), 256, 256, Format::R8G8B8A8);
    }
}
