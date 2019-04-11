// use crate::{backend::Renderer, objects::handle::Handle};
// use log::*;
// use std::error;
// use std::fmt;
// use std::sync::{Arc, Mutex};

// #[derive(Debug, Copy, Clone)]
// pub enum Format {
//     Rgba32,
//     Rgb24,
// }

// impl Format {
//     pub fn size(self) -> usize {
//         match self {
//             Format::Rgba32 => 4,
//             Format::Rgb24 => 3,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum TextureError {
//     AllocationFailed,
// }

// impl fmt::Display for TextureError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TextureError::AllocationFailed => write!(f, "could not allocate memory for texture"),
//         }
//     }
// }

// impl error::Error for TextureError {
//     fn description(&self) -> &str {
//         match self {
//             TextureError::AllocationFailed => "could not allocate memory for texture",
//         }
//     }

//     fn cause(&self) -> Option<&error::Error> {
//         None
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Texture2D<R>
// where
//     R: Renderer,
// {
//     pub renderer: Arc<Mutex<R>>,
//     pub width: usize,
//     pub height: usize,
//     pub format: Format,
//     pub handle: Handle,
// }

// impl<R: Renderer> Drop for Texture2D<R> {
//     fn drop(&mut self) {
//         debug!("dropping texture");
//         let mut r = self.renderer.lock().unwrap();
//         r.free_texture_2d(self.handle);
//     }
// }

// impl<R: Renderer> Texture2D<R> {
//     pub fn new(
//         renderer: Arc<Mutex<R>>,
//         width: usize,
//         height: usize,
//         format: Format,
//     ) -> Result<Arc<Texture2D<R>>, TextureError> {
//         debug!("create texture {}*{} ({:?})", width, height, format);

//         let mut r = renderer.lock().unwrap();
//         let handle = r.allocate_texture_2d(width, height, format);
//         let res = Texture2D {
//             renderer: renderer.clone(),
//             width,
//             height,
//             format,
//             handle,
//         };

//         Ok(Arc::new(res))
//     }

//     pub fn size(&self) -> usize {
//         self.width * self.height * self.format.size()
//     }
// }
