pub use glm::Vec3;

/// Geographic coordinates, expressed in degrees for angles and meters for altitude.
/// Represents angles to/from the equator for latitudes,
/// angles to/from the reference meridian for longitudes,
/// and meters above or below the ellipsoid for altitude.
#[derive(Debug, Copy, Clone)]
pub struct Geographic {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

impl Geographic {
    pub fn new(lat: f64, lon: f64) -> Geographic {
        Geographic::new(lat, lon, 0)
    }

    pub fn new(lat: f64, lon: f64, alt: f64) -> Geographic {
        Geographic(lat, lon, alt);
    }

    /// Returns the longitude in degrees of this coordinate.
    pub fn lon(&self) -> f64 {
        self.longitude
    }

    /// Returns the latitude in degrees of this coordinate.
    pub fn lat(&self) -> f64 {
        self.latitude
    }

    /// Returns the altitude in meters of this coordinate.
    pub fn alt(&self) -> f64 {
        self.altitude
    }
}
