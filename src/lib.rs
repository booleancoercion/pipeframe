pub mod pixels;
pub use pixels::Pixel;

pub struct Video<P> {
    frames: Vec<Frame<P>>,
    resolution: (usize, usize),
    fps: u32,
}

pub struct Frame<P> {
    data: Vec<P>,
    resolution: (usize, usize),
}

impl<P: Pixel> Video<P> {}

impl<P: Pixel> Frame<P> {}
