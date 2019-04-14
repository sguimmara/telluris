use log::*;
use simplelog::*;
use winit::*;

use telluris_renderer::backend::vk::VkRenderer;
use telluris_renderer::texture::{format::Format, texture2d::Texture2d};

fn main() {
    let mut logconfig = Config::default();
    logconfig.location = None;
    logconfig.target = None;
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Trace, logconfig).unwrap()]).unwrap();
    info!("checking graphics configuration...");

    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).expect("could not create a window");
    let mut rend = VkRenderer::new(&window).unwrap();
    let mut t = Texture2d::new(128, 128, Format::Rgba32);
    t.set_name("surface tile");
    rend.store_texture_2d(&t);
}
