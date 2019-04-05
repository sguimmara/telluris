use log::*;
use simplelog::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use telluris_renderer::{
    backend::vk::VkRenderer,
    objects::texture::{Format, Texture2D},
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

    for _i in 0..1000 {
        let _tex = Texture2D::new(mutex.clone(), 256, 256, Format::Rgba32);
    }
}
