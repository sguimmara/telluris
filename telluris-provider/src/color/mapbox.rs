use crate::color::ImageProvider;
use reqwest::r#async::Client;

#[allow(dead_code)]
pub struct MapboxImageProvider {
    pub token: String,
    client: Client,
}

impl Default for MapboxImageProvider {
    fn default() -> MapboxImageProvider {
        MapboxImageProvider {
            token: "pk.eyJ1Ijoic2d1aW1tYXJhIiwiYSI6ImNqYzEwOWo3ODA2bmozM3BteXU0eHFxejUifQ.7zpt42JYsR_vSl8rN4NKJw".to_string(),
            client: Client::new()
        }
    }
}

impl ImageProvider for MapboxImageProvider {
    fn name(&self) -> &str {
        "mapbox"
    }
    fn get_color_map(&self, x: u32, y: u32, z: u32) -> String {
        format!(
            "https://api.mapbox.com/v4/mapbox.satellite/{zoom}/{x}/{y}.{format}?access_token={token}",
            zoom=z,
            x=x,
            y=y,
            format="jpg70",
            token=self.token)
    }
}
