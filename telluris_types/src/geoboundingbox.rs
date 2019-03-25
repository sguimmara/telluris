use coordinates;

/// A geographic bounding box defines a volume
/// bounded by the min and max corners.
#[derive(Debug, Copy, Clone)]
struct GeoBoundingBox {
    min: Geographic,
    max: Geographic
}

impl GeoBoundingBox {
}