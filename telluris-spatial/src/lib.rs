//! Coordinates and spatial data structures.
//! Types prefixed with `Geo` are typically related to geographic coordinates,
//! where values are angles expressed in degrees. Types without this prefix are
//! typically related to a cartesian frame.
//! Some problems are more easily solved using geographic types, for example
//! modeling the earth's surface. Other problems are more naturally expressed
//! in a cartesian frame, such as horizon culling.

pub mod geographic;
pub mod geobounds;
pub mod index;
pub mod transformations;

#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate nalgebra_glm as glm;

#[cfg(test)]
#[macro_use]
extern crate approx;

