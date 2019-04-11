pub mod mapbox;

pub trait ImageProvider {
    fn name(&self) -> &str;
    // Returns a formatted URL to retrieve a color map.
    fn get_color_map(&self, x: u32, y: u32, z: u32) -> String;
}
