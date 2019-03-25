#[macro_use]
extern crate vulkano;

mod renderer;
pub use self::renderer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
