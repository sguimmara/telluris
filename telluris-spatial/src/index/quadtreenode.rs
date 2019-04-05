pub const MAX_DEPTH : u32 = 23;

#[derive(Debug)]
pub struct QuadtreeNode {
    row: u32,
    column: u32,
    depth: u32
}

impl QuadtreeNode {
    pub fn new(row: u32, column: u32, depth: u32) {
        assert!(depth <= MAX_DEPTH);

        unimplemented!();
    }
}