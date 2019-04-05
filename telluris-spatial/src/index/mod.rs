//! Spatial indexes.

pub mod geoquadtree;
use crate::geographic::Geographic;

// A division of a planar space
pub enum Quadrant {
    NorthWest = 0,
    NorthEast = 1,
    SouthWest = 2,
    SouthEast = 3,
}

pub trait GeoIndex {
    fn geo_index(&self) -> Geographic;
}