use quickcheck::{Arbitrary, Gen};

use rand::{self, Rng};

#[cfg(test)]
use approx::AbsDiffEq;

/// The westermost longitude on earth
pub const MIN_LON: f64 = -180.0;
/// The southernmost latitude on earth
pub const MIN_LAT: f64 = -90.0;
/// The eastermost longitude on earth
pub const MAX_LON: f64 = 180.0;
/// The northermost latitude on earth
pub const MAX_LAT: f64 = 90.0;
/// The minimal supported elevation, corresponding to the deepest point on Earth.
pub const MIN_ALT: f64 = -11_000.0;
/// The maximal supported elevation, way above the geostationary orbit.
pub const MAX_ALT: f64 = 50_000_000.0;

/// Geographic coordinates, expressed in degrees for angles and meters for elevation.
/// Represents angles to/from the equator for latitudes,
/// angles to/from the reference meridian for longitudes,
/// and meters above or below the ellipsoid for elevation.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Geographic {
    latitude: f64,
    longitude: f64,
    elevation: f64,
}

impl Geographic {
    /// Creates a geographic coordinate with the provided values.
    pub fn new(lat: f64, lon: f64, elevation: f64) -> Geographic {
        debug_assert!(lat >= MIN_LAT);
        debug_assert!(lat <= MAX_LAT);
        debug_assert!(lon <= MAX_LON);
        debug_assert!(lon >= MIN_LON);
        debug_assert!(elevation >= MIN_ALT);
        debug_assert!(elevation <= MAX_ALT);

        Geographic {
            latitude: lat,
            longitude: lon,
            elevation,
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

    /// Returns the elevation in meters of this coordinate.
    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    /// Returns a Geographic coordinate with elevation raised (or lowered) by
    /// the specified value. The new elevation is clamped in the domain specified
    /// by `MIN_ALT` and `MAX_ALT`.
    pub fn raise(self, elev: f64) -> Self {
        Geographic {
            latitude: self.latitude,
            longitude: self.longitude,
            elevation: num::clamp(self.elevation() + elev, MIN_ALT, MAX_ALT)
        }
    }

    /// Returns a Geographic coordinate with elevation set at zero.
    pub fn flatten(self) -> Self {
        Geographic {
            latitude: self.latitude,
            longitude: self.longitude,
            elevation: 0.0,
        }
    }
}

impl Arbitrary for Geographic {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let lon = g.gen_range(MIN_LON, MAX_LON);
        let lat = g.gen_range(MIN_LAT, MAX_LAT);
        let elevation = g.gen_range(MIN_ALT, MAX_ALT);

        Self::new(lat, lon, elevation)
    }
}

#[cfg(test)]
impl AbsDiffEq for Geographic {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <f64 as AbsDiffEq>::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <f64 as AbsDiffEq>::Epsilon) -> bool {
        f64::abs_diff_eq(&self.latitude, &other.latitude, epsilon)
            && f64::abs_diff_eq(&self.longitude, &other.longitude, epsilon)
            && f64::abs_diff_eq(&self.elevation, &other.elevation, epsilon)
    }
}

#[cfg(test)]
mod test {
    use crate::geographic::*;
    use num::clamp;
    #[quickcheck]
    fn new_creates_correct_geographic(lat: f64, lon: f64, elev: f64) -> bool {
        let lat = clamp(lat, MIN_LAT, MAX_LAT);
        let lon = clamp(lon, MIN_LON, MAX_LON);
        let elev = clamp(elev, MIN_ALT, MAX_ALT);

        let geo = Geographic::new(lat, lon, elev);

        geo.lat() == lat && geo.lon() == lon && geo.elevation() == elev
    }

    #[quickcheck]
    fn raise_returns_correct_values(p: Geographic, elev: f64) -> bool {

        let p2 = p.raise(elev);
        p2.lat() == p.lat() && p2.lon() == p.lon() && p2.elevation() == (p.elevation() + elev)
    }

    #[quickcheck]
    fn flatten_returns_correct_values(p: Geographic) -> bool {
        let f = p.flatten();
        f.elevation() == 0.0 && f.lat() == p.lat() && f.lon() == p.lon()
    }
}
