use crate::spatial::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// A geographic region defines a volume bounded by the min and max corners.
#[derive(Debug, Copy, Clone)]
pub struct Region {
    min: Geographic,
    max: Geographic,
}

impl Region {
    /// Returns a region encompassing the whole world.
    /// Any coordinate outside those bounds are unsupported.
    pub fn world() -> Region {
        Region {
            min: Geographic::new(MIN_LAT, MIN_LON, MIN_ALT),
            max: Geographic::new(MAX_LAT, MAX_LON, MAX_ALT),
        }
    }

    /// Creates a region with the specified min and max values.
    pub fn new(min: Geographic, max: Geographic) -> Region {
        Region { min, max }
    }

    /// Returns the geographic center of this region
    pub fn center(&self) -> Geographic {
        self.sample(0.5f64, 0.5f64, 0.5f64)
    }

    /// Returns the western edge (or minimal longitude) of this region
    pub fn west(&self) -> f64 {
        self.min.lon()
    }

    /// Returns the eastern edge (or maximal longitude) of this region
    pub fn east(&self) -> f64 {
        self.max.lon()
    }

    /// Returns the southern edge (or minimal latitude) of this region
    pub fn south(&self) -> f64 {
        self.min.lat()
    }

    /// Returns the northern edge (or maximal latitude) of this region
    pub fn north(&self) -> f64 {
        self.max.lat()
    }

    /// Returns the minimal altitude of this region
    pub fn floor(&self) -> f64 {
        self.min.alt()
    }

    /// Returns the maximal altitude of this region
    pub fn top(&self) -> f64 {
        self.max.alt()
    }

    /// Returns the longitude span (in degrees) of this region
    pub fn span_lon(&self) -> f64 {
        self.max.lon() - self.min.lon()
    }

    /// Returns the latitude span (in degrees) of this region
    pub fn span_lat(&self) -> f64 {
        self.max.lat() - self.min.lat()
    }

    /// Returns the height (in meters) of this region
    pub fn height(&self) -> f64 {
        self.max.alt() - self.min.alt()
    }

    /// Returns true if the two regiones intersect.
    pub fn intersects(&self, other: &Region) -> bool {
        (self.min.lon() < other.max.lon())
            && (self.max.lon() > other.min.lon())
            && (self.min.lat() < other.max.lat())
            && (self.max.lat() > other.min.lat())
            && (self.min.alt() < other.max.alt())
            && (self.max.alt() > other.min.alt())
    }

    /// Returns true if the coordinate is contained in this region, including its borders.
    pub fn contains(&self, v: Geographic) -> bool {
        v.lat() <= self.max.lat()
            && v.lon() <= self.max.lon()
            && v.lat() >= self.min.lat()
            && v.lon() >= self.min.lon()
            && v.alt() >= self.min.alt()
            && v.alt() <= self.max.alt()
    }

    /// Returns the geographic coordinate sampled at the given (x, y, z) coordinates.
    /// The given values must be normalized (e.g each value must be in the [0, 1] range)
    pub fn sample(&self, x: f64, y: f64, z: f64) -> Geographic {
        let lon = self.min.lon() + self.span_lon() * x;
        let lat = self.min.lat() + self.span_lat() * y;
        let alt = self.min.alt() + self.height() * z;

        Geographic::new(lat, lon, alt)
    }
}

#[cfg(test)]
impl Arbitrary for Region {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let a = <Geographic as Arbitrary>::arbitrary(g);
        let b = <Geographic as Arbitrary>::arbitrary(g);

        let minlon = a.lon().min(b.lon());
        let maxlon = a.lon().max(b.lon());
        let minlat = a.lat().min(b.lat());
        let maxlat = a.lat().max(b.lat());
        let minalt = a.alt().min(b.alt());
        let maxalt = a.alt().min(b.alt());

        let min = Geographic::new(minlat, minlon, minalt);
        let max = Geographic::new(maxlat, maxlon, maxalt);

        Region::new(min, max)
    }
}

#[cfg(test)]
mod test {
    use crate::spatial::Geographic;
    use crate::spatial::Region;
    use rand::Rng;

    #[quickcheck]
    fn world_contains_all_valid_coordinates(v: Geographic) -> bool {
        Region::world().contains(v)
    }

    #[quickcheck]
    fn contains_its_corners(g: Region) -> bool {
        g.contains(Geographic::new(g.south(), g.west(), g.floor()))
            && g.contains(Geographic::new(g.north(), g.west(), g.floor()))
            && g.contains(Geographic::new(g.south(), g.east(), g.floor()))
            && g.contains(Geographic::new(g.north(), g.east(), g.floor()))
            && g.contains(Geographic::new(g.south(), g.west(), g.top()))
            && g.contains(Geographic::new(g.north(), g.west(), g.top()))
            && g.contains(Geographic::new(g.south(), g.east(), g.top()))
            && g.contains(Geographic::new(g.north(), g.east(), g.top()))
    }

    #[quickcheck]
    fn contains_returns_true_for_any_point_inside(g: Region) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0f64, 1.0f64);
        let y = rng.gen_range(0.0f64, 1.0f64);
        let z = rng.gen_range(0.0f64, 1.0f64);
        g.contains(g.sample(x, y, z))
    }
}