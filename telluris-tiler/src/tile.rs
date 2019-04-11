use specs::{Component, HashMapStorage};
use telluris_spatial::geobounds::GeoBounds;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// The Tile is the fundamental spatial element, serving as the entry point
/// of the rendering pipeline.
#[derive(Clone)]
pub struct Tile {
    bounds: GeoBounds,
}

impl Component for Tile {
    type Storage = HashMapStorage<Self>;
}

impl Tile {
    pub fn new(bounds: GeoBounds) -> Self {
        Tile { bounds }
    }
    pub fn bounds(&self) -> GeoBounds {
        self.bounds
    }
}

#[cfg(test)]
impl Arbitrary for Tile {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let bounds = <GeoBounds as Arbitrary>::arbitrary(g);

        Self::new(bounds)
    }
}

#[cfg(test)]
mod test {
    use crate::tile::Tile;
    use telluris_spatial::geobounds::GeoBounds;

    #[quickcheck]
    fn constructor_assigns_correct_bounds(b: GeoBounds) -> bool {
        let t = Tile::new(b);
        t.bounds() == b
    }
}
