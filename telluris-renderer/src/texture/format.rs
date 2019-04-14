#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Format {
    Rgba32,
    Rgb24,
}

impl Format {
    pub fn bpp(self) -> u32 {
        match self {
            Format::Rgba32 => 4,
            Format::Rgb24 => 3,
        }
    }
}
