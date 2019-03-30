use crate::spatial::*;
use num::{clamp};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use approx::{AbsDiffEq};

/// A geographic region defines a volume bounded by the min and max corners.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

    /// Returns the union between this region and the other.
    pub fn expand(self, other: &Region) -> Region {

        let new_min_lat = self.min.lat().min(other.min.lat());
        let new_max_lat = self.max.lat().max(other.max.lat());
        let new_min_lon = self.min.lon().min(other.min.lon());
        let new_max_lon = self.max.lon().max(other.max.lon());
        let new_min_alt = self.min.alt().min(other.min.alt());
        let new_max_alt = self.max.alt().max(other.max.alt());

        Region {
            min: Geographic::new(new_min_lat, new_min_lon, new_min_alt),
            max: Geographic::new(new_max_lat, new_max_lon, new_max_alt)
        }
    }

    /// Returns the union of the two regions.
    pub fn union(a: &Region, b: &Region) -> Region {
        a.expand(b)
    }

    /// Grows the region in horizontal and vertical directions with the given values, in degrees.
    /// Altitudes are untouched.
    pub fn grow(self, horizontal: f64, vertical: f64) -> Region {

        
        let new_min_lat = clamp(self.min.lat() - vertical as f64, MIN_LAT, MAX_LAT);
        let new_max_lat = clamp(self.max.lat() + vertical as f64, MIN_LAT, MAX_LAT);
        let new_min_lon = clamp(self.min.lon() - horizontal as f64, MIN_LON, MAX_LON);
        let new_max_lon = clamp(self.max.lon() + horizontal as f64, MIN_LON, MAX_LON);

        Region {
            min: Geographic::new(new_min_lat, new_min_lon, self.min.alt()),
            max: Geographic::new(new_max_lat, new_max_lon, self.max.alt())
        }
    }

    /// Shrinks the region in horizontal and vertical directions with the given values, in degrees.
    /// Altitudes are untouched. If the shrink factor would make two edges cross each other, their center line
    /// is used instead, to avoid inverting the region area.
    pub fn shrink(self, horizontal: f64, vertical: f64) -> Region {

        let mut new_min_lat = self.min.lat() + vertical as f64;
        let mut new_max_lat = self.max.lat() - vertical as f64;
        let mut new_min_lon = self.min.lon() + horizontal as f64;
        let mut new_max_lon = self.max.lon() - horizontal as f64;

        let center = self.center();

        if new_min_lat > new_max_lat {
            new_max_lat = center.lat();
            new_min_lat = center.lat();
        }

        if new_min_lon > new_max_lon {
            new_max_lon = center.lon();
            new_min_lon = center.lon();
        }

        let new_min = Geographic::new(new_min_lat, new_min_lon, self.min.alt());
        let new_max = Geographic::new(new_max_lat, new_max_lon, self.max.alt());

        Region {
            min: new_min,
            max: new_max
        }
    }

    /// Fills the provided grid with equally spaced samples of this region.
    /// Samples are laid out row after row in the grid, starting at the southwest corner.
    pub fn grid(&self, x_count: u32, y_count: u32, grid: &mut [Geographic]) {
        unimplemented!();
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
impl AbsDiffEq for Region {
    type Epsilon = <Geographic as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <Geographic as AbsDiffEq>::Epsilon {
        Geographic::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <Geographic as AbsDiffEq>::Epsilon) -> bool {
        Geographic::abs_diff_eq(&self.min, &other.min, epsilon) &&
        Geographic::abs_diff_eq(&self.max, &other.max, epsilon)
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
    fn shrink_then_grow_with_same_values_produces_the_same_region(g: Region) -> bool {
        let original = g;

        let x = g.span_lon() * 0.3;
        let y = g.span_lat() * 0.3;

        let result = g.shrink(x, y).grow(x, y);

        abs_diff_eq!(result.min, original.min, epsilon=0.001)
    }

    #[quickcheck]
    fn sample_in_the_0_1_range_never_returns_an_outside_point(g: Region) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0f64, 1.0f64);
        let y = rng.gen_range(0.0f64, 1.0f64);
        let z = rng.gen_range(0.0f64, 1.0f64);
        let p = g.sample(x, y, z);

        p.lat() <= g.max.lat() && p.lat() >= g.min.lat() &&
        p.lon() <= g.max.lon() && p.lon() >= g.min.lon() &&
        p.alt() <= g.max.alt() && p.alt() >= g.min.alt()
    }

    #[quickcheck]
    fn contains_returns_true_for_any_point_inside(g: Region) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0f64, 1.0f64);
        let y = rng.gen_range(0.0f64, 1.0f64);
        let z = rng.gen_range(0.0f64, 1.0f64);
        g.contains(g.sample(x, y, z))
    }

    #[quickcheck]
    fn expand_produces_correct_values(r1: Region, r2: Region) -> bool {
        let r3 = r1.expand(&r2);

        abs_diff_eq!(r3.max.lat(), r1.max.lat().max(r2.max.lat()), epsilon=0.001) &&
        abs_diff_eq!(r3.max.lon(), r1.max.lon().max(r2.max.lon()), epsilon=0.001) &&
        abs_diff_eq!(r3.min.lat(), r1.min.lat().min(r2.min.lat()), epsilon=0.001) &&
        abs_diff_eq!(r3.min.lon(), r1.min.lon().min(r2.min.lon()), epsilon=0.001) &&
        abs_diff_eq!(r3.min.alt(), r1.min.alt().min(r2.min.alt()), epsilon=0.001) &&
        abs_diff_eq!(r3.max.alt(), r1.max.alt().max(r2.max.alt()), epsilon=0.001)
    }
}
