use log::*;
use simplelog::*;
use std::fs::File;
use telluris_renderer::vk::Renderer;

use winit::*;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
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
    let rend = Renderer::new(&window);
    match rend {
        Ok(_) => info!("your device is compatible with Vulkan."),
        Err(e) => error!("no compatible configuration found! \
Please check that your graphics driver is up to date an compatible with Vulkan. ({:?})", e),
    };
}
