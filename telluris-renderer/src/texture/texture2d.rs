use super::format::Format;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use quickcheck::{Arbitrary, Gen};
use std::fmt;

lazy_static! {
    static ref TEXTURE_ID: RelaxedCounter = RelaxedCounter::new(0);
}

#[derive(Debug, Clone)]
pub struct Texture2d {
    id: usize,
    name: String,
    width: u32,
    height: u32,
    format: Format,
    data: Vec<u8>,
}

impl Texture2d {
    pub fn new(width: u32, height: u32, format: Format) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let id = TEXTURE_ID.inc();
        let size = width * height * format.bpp();

        Texture2d {
            id,
            name: "".to_string(),
            width,
            height,
            format,
            data: vec![0; size as usize],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn load(&mut self, data: &[u8]) {
        assert!(data.len() == self.size());
        self.data = data.to_vec();
    }
}

impl fmt::Display for Texture2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<Texture2d 0x{:x} '{}' {:?} {}*{}, {} bytes>",
            self.id,
            self.name(),
            self.format,
            self.width,
            self.height,
            self.size()
        )
    }
}

impl Arbitrary for Texture2d {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let width = 1 + (<u32 as Arbitrary>::arbitrary(g) % 1024);
        let height = 1 + (<u32 as Arbitrary>::arbitrary(g) % 1024);

        Self::new(width, height, Format::Rgba32)
    }
}

#[cfg(test)]
mod test {
    use crate::texture::format::Format;
    use crate::texture::texture2d::Texture2d;

    #[test]
    fn new_assigns_correct_fields() {
        let t = Texture2d::new(128, 37, Format::Rgba32);
        assert!(t.width() == 128);
        assert!(t.height() == 37);
        assert!(t.format() == Format::Rgba32);
    }

    #[test]
    fn load() {
        let mut t = Texture2d::new(1, 1, Format::Rgba32);
        let mut data: [u8; 4] = [0; 4];
        data[0] = 127;
        data[1] = 187;
        data[2] = 1;
        data[3] = 255;

        t.load(&data);

        let tdata = t.data();
        for i in 0..4 {
            assert!(data[i] == tdata[i]);
        }
    }
}
