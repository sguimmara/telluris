use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HandleType {
    Unallocated,
    Texture2D
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Handle {
    pub id: usize,
    pub ty: HandleType
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:#X?} ({:?}>)", self.id, self.ty)
    }
}

impl Handle {
    pub fn null() -> Self {
        Handle {id: 0, ty: HandleType::Unallocated }
    }
    pub fn texture_2d(id: usize) -> Self {
        Handle { id, ty: HandleType::Texture2D}
    }
}