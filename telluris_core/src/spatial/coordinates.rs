pub use glm::Vec3;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use rand::{self, Rng};

#[cfg(test)]
use approx::{AbsDiffEq};

/// The westermost longitude on earth
pub const MIN_LON: f64 = -180.0;
/// The southernmost latitude on earth
pub const MIN_LAT: f64 = -90.0;
/// The eastermost longitude on earth
pub const MAX_LON: f64 = 180.0;
/// The northermost latitude on earth
pub const MAX_LAT: f64 = 90.0;
/// The minimal supported altitude, corresponding to the deepest point on Earth.
pub const MIN_ALT: f64 = -11_000.0;
/// The maximal supported altitude, way above the geostationary orbit.
pub const MAX_ALT: f64 = 50_000_000.0;

/// Geographic coordinates, expressed in degrees for angles and meters for altitude.
/// Represents angles to/from the equator for latitudes,
/// angles to/from the reference meridian for longitudes,
/// and meters above or below the ellipsoid for altitude.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Geographic {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

impl Geographic {
    /// Creates a geographic coordinate with the provided values.
    pub fn new(lat: f64, lon: f64, alt: f64) -> Geographic {
        debug_assert!(lat >= MIN_LAT);
        debug_assert!(lat <= MAX_LAT);
        debug_assert!(lon <= MAX_LON);
        debug_assert!(lon >= MIN_LON);
        debug_assert!(alt >= MIN_ALT);
        debug_assert!(alt <= MAX_ALT);

        Geographic {
            latitude: lat,
            longitude: lon,
            altitude: alt,
        }
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

#[cfg(test)]
impl Arbitrary for Geographic {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let lon = g.gen_range(MIN_LON, MAX_LON);
        let lat = g.gen_range(MIN_LAT, MAX_LAT);
        let alt = g.gen_range(MIN_ALT, MAX_ALT);

        Self::new(lat, lon, alt)
    }
}

#[cfg(test)]
impl AbsDiffEq for Geographic {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        f64::abs_diff_eq(&self.latitude, &other.latitude, epsilon) &&
        f64::abs_diff_eq(&self.longitude, &other.longitude, epsilon) &&
        f64::abs_diff_eq(&self.altitude, &other.altitude, epsilon)
    }
}

