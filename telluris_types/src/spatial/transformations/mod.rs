use crate::spatial::{Geographic, Vec3};

pub mod ecef;

/// Provides coordinate transformation between geographic coordinates
/// and cartesian coordinates.
pub trait SpatialReference {
    /// Converts geographic coordinates into cartesian coordinates.
    fn convert(self: &Self, geo: Geographic) -> Vec3;

    /// Returns the normal vector of the given coordinate.
    fn normal(self: &Self, geo: Geographic) -> Vec3;
}
