use crate::spatial::{Geographic, Vec3};
use std::fmt::Debug;

pub mod ecef;

/// Provides coordinate transformation between geographic coordinates
/// and cartesian coordinates.
pub trait SpatialReference: Debug {
    /// Converts geographic coordinates into cartesian coordinates.
    fn convert(self: &Self, geo: Geographic) -> Vec3;

    /// Returns the normal vector of the given coordinate.
    fn normal(self: &Self, geo: Geographic) -> Vec3;
}
