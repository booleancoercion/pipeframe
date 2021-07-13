pub mod pixels;
use pixels::Rgb;

pub struct Video<P> {
    frames: Vec<Frame<P>>,
    resolution: (usize, usize),
    fps: u32,
}

pub struct Frame<P> {
    data: Vec<P>,
    resolution: (usize, usize),
}

impl<P: Into<Rgb>> Video<P> {}

impl<P: Into<Rgb>> Frame<P> {}
