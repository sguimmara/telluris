use crate::coordinates::{Geographic, Vec3};
use crate::spatialreference::SpatialReference;
use glm::DVec3;

/// The length of the semi major axis, in meters, in the WGS 84 system.
pub const WGS84_SEMI_MAJOR_AXIS: f64 = 6_378_137.0;

/// The length of the semi minor axis, in meters, in the WGS 84 system.
pub const WGS84_SEMI_MINOR_AXIS: f64 = 6_356_752.314_245;

const WGS84_RADII_SQUARED: (f64, f64, f64) = (
    WGS84_SEMI_MINOR_AXIS * WGS84_SEMI_MINOR_AXIS,
    WGS84_SEMI_MAJOR_AXIS * WGS84_SEMI_MAJOR_AXIS,
    WGS84_SEMI_MAJOR_AXIS * WGS84_SEMI_MAJOR_AXIS,
);

/// Earth-centered, earth-fixed referential system.
/// North is positive `Z`, the intersection between the prime meridian
/// and the equator (0°N, 0°E) lies on the positive `X` axis,
/// and the intersection between the 90°E meridian and the equator (0°N, 90°E)
/// lies on the positive `Y` axis.
pub struct ECEF {}

impl SpatialReference for ECEF {
    fn convert(self: &Self, geo: Geographic) -> Vec3 {
        // Implementation taken from the book "3D Engine design
        // for virtual globes", by Patrick Cozzi and Kevin Ring.

        let cos_lat = geo.lat().cos();
        let n = DVec3::new(
            cos_lat * geo.lon().cos(),
            cos_lat * geo.lon().sin(),
            geo.lat().sin(),
        );
        let k = DVec3::new(
            WGS84_RADII_SQUARED.0 * n.x,
            WGS84_RADII_SQUARED.1 * n.y,
            WGS84_RADII_SQUARED.2 * n.z,
        );
        let gamma = (k.x * n.x + k.y * n.y + k.z * n.z).sqrt();
        let surface = k / gamma;
        let result = surface + (geo.alt() * n);

        Vec3::new(result.x as f32, result.z as f32, result.y as f32)
    }

    fn normal(self: &Self, geo: Geographic) -> Vec3 {
        let cos_lat = geo.lat().cos() as f32;

        Vec3::new(
            cos_lat * geo.lon().cos() as f32,
            cos_lat * geo.lon().sin() as f32,
            geo.lat().sin() as f32,
        )
    }
}
