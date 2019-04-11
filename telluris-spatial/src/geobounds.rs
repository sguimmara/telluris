use num::clamp;
use crate::geographic::*;

use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use approx::AbsDiffEq;

/// Represents a volume bounded by two geographic corners.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct GeoBounds {
    min: Geographic,
    max: Geographic,
}

impl GeoBounds {
    /// Returns bounds encompassing the whole world.
    /// Any coordinate outside those bounds are unsupported.
    pub fn world() -> Self {
        GeoBounds {
            min: Geographic::new(MIN_LAT, MIN_LON, MIN_ALT),
            max: Geographic::new(MAX_LAT, MAX_LON, MAX_ALT),
        }
    }

    /// Returns bounds encompassing the whole ellipsoid surface.
    pub fn surface() -> Self {
        Self::world().flatten()
    }

    /// Creates bounds with the specified min and max values.
    pub fn new(min: Geographic, max: Geographic) -> Self {
        GeoBounds { min, max }
    }

    /// Returns bounds with altitudes set at zero.
    pub fn flatten(self) -> Self {
        GeoBounds {
            min: self.min.flatten(),
            max: self.max.flatten(),
        }
    }

    /// Returns the union between this bounds and the other.
    pub fn expand(self, other: &GeoBounds) -> Self {
        let new_min_lat = self.south().min(other.south());
        let new_max_lat = self.north().max(other.north());
        let new_min_lon = self.west().min(other.west());
        let new_max_lon = self.east().max(other.east());
        let new_min_alt = self.floor().min(other.floor());
        let new_max_alt = self.top().max(other.top());

        GeoBounds {
            min: Geographic::new(new_min_lat, new_min_lon, new_min_alt),
            max: Geographic::new(new_max_lat, new_max_lon, new_max_alt),
        }
    }

    /// Returns the union of the two boundss.
    pub fn union(a: &GeoBounds, b: &GeoBounds) -> Self {
        a.expand(b)
    }

    /// Grows the bounds in horizontal and vertical directions with the given values, in degrees.
    /// Altitudes are untouched.
    pub fn grow(self, horizontal: f64, vertical: f64) -> Self {
        let new_min_lat = clamp(self.south() - vertical as f64, MIN_LAT, MAX_LAT);
        let new_max_lat = clamp(self.north() + vertical as f64, MIN_LAT, MAX_LAT);
        let new_min_lon = clamp(self.west() - horizontal as f64, MIN_LON, MAX_LON);
        let new_max_lon = clamp(self.east() + horizontal as f64, MIN_LON, MAX_LON);

        GeoBounds {
            min: Geographic::new(new_min_lat, new_min_lon, self.floor()),
            max: Geographic::new(new_max_lat, new_max_lon, self.top()),
        }
    }

    /// Shrinks the bounds in horizontal and vertical directions with the given values, in degrees.
    /// Altitudes are untouched.
    /// If the shrink factor would make two edges cross each other, their center line
    /// is used instead, to avoid inverting the bounds area.
    pub fn shrink(self, horizontal: f64, vertical: f64) -> Self {
        assert!(horizontal >= 0.0);
        assert!(vertical >= 0.0);

        let mut new_min_lat = self.south() + vertical as f64;
        let mut new_max_lat = self.north() - vertical as f64;
        let mut new_min_lon = self.west() + horizontal as f64;
        let mut new_max_lon = self.east() - horizontal as f64;

        let center = self.center();

        if new_min_lat > new_max_lat {
            new_max_lat = center.lat();
            new_min_lat = center.lat();
        }

        if new_min_lon > new_max_lon {
            new_max_lon = center.lon();
            new_min_lon = center.lon();
        }

        let new_min = Geographic::new(new_min_lat, new_min_lon, self.floor());
        let new_max = Geographic::new(new_max_lat, new_max_lon, self.top());

        GeoBounds {
            min: new_min,
            max: new_max,
        }
    }

    /// Fills the provided slice with equally spaced samples of this bounds,
    /// arranged in a grid pattern. Samples are laid out row after row, from west
    /// to east, then south to north.
    pub fn grid(&self, grid: &mut [Geographic], x_count: usize, y_count: usize) {
        assert!(x_count > 1);
        assert!(y_count > 1);
        assert!(
            grid.len() >= (x_count * y_count),
            "the provided slice is not big enough to store the grid"
        );

        let x_step = 1.0 / (x_count - 1) as f64;
        let y_step = 1.0 / (y_count - 1) as f64;

        let mut k = 0;
        for y in 0..y_count {
            for x in 0..x_count {
                let u = (x as f64) * x_step;
                let v = (y as f64) * y_step;
                grid[k] = self.sample(u, v, 0.0);
                k += 1;
            }
        }
    }

    /// Returns the geographic center of this bounds
    pub fn center(&self) -> Geographic {
        self.sample(0.5f64, 0.5f64, 0.5f64)
    }

    /// Returns the western edge (or minimal longitude) of this bounds
    pub fn west(&self) -> f64 {
        self.min.lon()
    }

    /// Returns the eastern edge (or maximal longitude) of this bounds
    pub fn east(&self) -> f64 {
        self.max.lon()
    }

    /// Returns the southern edge (or minimal latitude) of this bounds
    pub fn south(&self) -> f64 {
        self.min.lat()
    }

    /// Returns the northern edge (or maximal latitude) of this bounds
    pub fn north(&self) -> f64 {
        self.max.lat()
    }

    /// Returns the minimal altitude of this bounds
    pub fn floor(&self) -> f64 {
        self.min.elevation()
    }

    /// Returns the maximal altitude of this bounds
    pub fn top(&self) -> f64 {
        self.max.elevation()
    }

    /// Returns the longitude span (in degrees) of this bounds. That is, the
    /// difference between the `east` and `west` values.
    pub fn span_lon(&self) -> f64 {
        self.east() - self.west()
    }

    /// Returns the latitude span (in degrees) of this bounds. That is, the
    /// difference between the `north` and `south` values.
    pub fn span_lat(&self) -> f64 {
        self.north() - self.south()
    }

    /// Returns the height (in meters) of this bounds. The height is defined
    /// by the difference between the `top` and `floor` values.
    pub fn height(&self) -> f64 {
        self.top() - self.floor()
    }

    /// Returns true if the two bounds intersect.
    pub fn intersects(&self, other: &GeoBounds) -> bool {
        (self.west() < other.east())
            && (self.east() > other.west())
            && (self.south() < other.north())
            && (self.north() > other.south())
            && (self.floor() < other.top())
            && (self.top() > other.floor())
    }

    /// Returns true if the coordinate is contained in this bounds, including its borders.
    pub fn contains(&self, v: Geographic) -> bool {
        v.lat() <= self.north()
            && v.lon() <= self.east()
            && v.lat() >= self.south()
            && v.lon() >= self.west()
            && v.elevation() >= self.floor()
            && v.elevation() <= self.top()
    }

    /// Returns the geographic coordinate sampled at the given (x, y, z) coordinates.
    /// The given values must be normalized (e.g each value must be in the [0, 1] range)
    pub fn sample(&self, x: f64, y: f64, z: f64) -> Geographic {
        let lon = self.west() + self.span_lon() * x;
        let lat = self.south() + self.span_lat() * y;
        let alt = self.floor() + self.height() * z;

        Geographic::new(lat, lon, alt)
    }
}

impl Arbitrary for GeoBounds {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let a = <Geographic as Arbitrary>::arbitrary(g);
        let b = <Geographic as Arbitrary>::arbitrary(g);

        let minlon = a.lon().min(b.lon());
        let maxlon = a.lon().max(b.lon());
        let minlat = a.lat().min(b.lat());
        let maxlat = a.lat().max(b.lat());
        let minalt = a.elevation().min(b.elevation());
        let maxalt = a.elevation().min(b.elevation());

        let min = Geographic::new(minlat, minlon, minalt);
        let max = Geographic::new(maxlat, maxlon, maxalt);

        GeoBounds::new(min, max)
    }
}

#[cfg(test)]
impl AbsDiffEq for GeoBounds {
    type Epsilon = <Geographic as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> <Geographic as AbsDiffEq>::Epsilon {
        Geographic::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: <Geographic as AbsDiffEq>::Epsilon) -> bool {
        Geographic::abs_diff_eq(&self.min, &other.min, epsilon)
            && Geographic::abs_diff_eq(&self.max, &other.max, epsilon)
    }
}

#[cfg(test)]
mod test {
    use crate::geographic::*;
    use crate::geobounds::*;
    use rand::Rng;

    #[quickcheck]
    fn world_contains_all_valid_coordinates(v: Geographic) -> bool {
        GeoBounds::world().contains(v)
    }

    #[quickcheck]
    fn contains_its_corners(g: GeoBounds) -> bool {
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
    fn shrink_then_grow_with_same_values_produces_the_same_bounds(g: GeoBounds) -> bool {
        let original = g;

        let x = g.span_lon() * 0.3;
        let y = g.span_lat() * 0.3;

        let result = g.shrink(x, y).grow(x, y);

        abs_diff_eq!(result.min, original.min, epsilon = 0.001)
    }

    #[quickcheck]
    fn sample_in_the_0_1_range_never_returns_an_outside_point(g: GeoBounds) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0f64, 1.0f64);
        let y = rng.gen_range(0.0f64, 1.0f64);
        let z = rng.gen_range(0.0f64, 1.0f64);
        let sample = g.sample(x, y, z);

        sample.lat() <= g.north()
            && sample.lat() >= g.south()
            && sample.lon() <= g.east()
            && sample.lon() >= g.west()
            && sample.elevation() <= g.top()
            && sample.elevation() >= g.floor()
    }

    #[quickcheck]
    fn contains_returns_true_for_any_point_inside(g: GeoBounds) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0f64, 1.0f64);
        let y = rng.gen_range(0.0f64, 1.0f64);
        let z = rng.gen_range(0.0f64, 1.0f64);
        g.contains(g.sample(x, y, z))
    }

    #[quickcheck]
    fn expand_produces_correct_values(r1: GeoBounds, r2: GeoBounds) -> bool {
        let r3 = r1.expand(&r2);

        abs_diff_eq!(r3.north(), r1.north().max(r2.north()), epsilon = 0.001)
            && abs_diff_eq!(r3.east(), r1.east().max(r2.east()), epsilon = 0.001)
            && abs_diff_eq!(r3.south(), r1.south().min(r2.south()), epsilon = 0.001)
            && abs_diff_eq!(r3.west(), r1.west().min(r2.west()), epsilon = 0.001)
            && abs_diff_eq!(r3.floor(), r1.floor().min(r2.floor()), epsilon = 0.001)
            && abs_diff_eq!(r3.top(), r1.top().max(r2.top()), epsilon = 0.001)
    }

    #[quickcheck]
    fn flatten_returns_correct_values(p: Geographic) -> bool {
        let f = p.flatten();
        f.elevation() == 0.0 && f.lat() == p.lat() && f.lon() == p.lon()
    }

    #[test]
    fn grid_returns_correct_values_for_2x2_grid() {
        let b = GeoBounds::world().flatten();
        let mut grid = vec![Geographic::default(); 4];
        b.grid(&mut grid, 2, 2);

        assert_abs_diff_eq!(
            grid[0],
            Geographic::new(MIN_LAT, MIN_LON, 0.0),
            epsilon = 0.001
        );
        assert_abs_diff_eq!(
            grid[1],
            Geographic::new(MIN_LAT, MAX_LON, 0.0),
            epsilon = 0.001
        );
        assert_abs_diff_eq!(
            grid[2],
            Geographic::new(MAX_LAT, MIN_LON, 0.0),
            epsilon = 0.001
        );
        assert_abs_diff_eq!(
            grid[3],
            Geographic::new(MAX_LAT, MAX_LON, 0.0),
            epsilon = 0.001
        );
    }

    #[quickcheck]
    fn all_points_from_grid_are_contained_in_the_bounds(
        r: GeoBounds,
        x_count: u8,
        y_count: u8,
    ) -> bool {
        let x_count = (x_count as usize) + 2;
        let y_count = (y_count as usize) + 2;
        let mut array = vec![Geographic::default(); x_count * y_count];

        r.grid(&mut array[..], x_count, y_count);

        // take into account floating point precision issues by very slightly
        // growing the bounds.
        let r1 = r.grow(r.span_lon() * 0.00001, r.span_lat() * 0.00001);

        for p in array {
            assert!(r1.contains(p));
        }

        true
    }
}
