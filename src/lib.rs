pub mod pixels;

use pixels::Rgb;

use std::io::{BufWriter, Write};
use std::ops::{Index, IndexMut};
use std::process::{Child, ChildStdin, Command, Stdio};

/// Represents an entire video to be piped into ffmpeg.
pub struct Video<P> {
    buffer: Frame<P>,
    resolution: (usize, usize),
    fps: u32,
    child: Child,
    stdin: BufWriter<ChildStdin>,
}

/// Represents a single frame that belongs to a Video struct.
pub struct Frame<P> {
    data: Vec<P>,
    resolution: (usize, usize),
}

impl<P: Default> Video<P>
where
    for<'a> &'a P: Into<Rgb>,
{
    /// Creates a new empty video with the given resolution and FPS.
    pub fn new<S: std::fmt::Display>(resolution: (usize, usize), fps: u32, filename: S) -> Self {
        let (x, y) = resolution;
        let mut child = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgb24",
                "-video_size",
                &format!("{}x{}", x, y),
                "-framerate",
                &fps.to_string(),
                "-i",
                "-",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-an",
                &format!("{}.mp4", filename),
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("couldn't spawn child process for ffmpeg");

        let stdin = child
            .stdin
            .take()
            .expect("couldn't get handle to child stdin");

        let stdin = BufWriter::new(stdin);

        Self {
            buffer: Frame::new(resolution),
            resolution,
            fps,
            child,
            stdin,
        }
    }

    /// Resets the frame buffer and returns a mutable reference to it.
    pub fn reset_frame(&mut self) -> &mut Frame<P> {
        self.buffer.data.fill_with(Default::default);

        &mut self.buffer
    }

    /// Returns a mutable reference to the current frame buffer without modifying it.
    pub fn get_frame_mut(&mut self) -> &mut Frame<P> {
        &mut self.buffer
    }

    /// Returns this video's resolution as an (x, y) tuple.
    pub fn get_resolution(&self) -> (usize, usize) {
        self.resolution
    }

    /// Returns this video's framerate in FPS (Frames Per Second).
    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    /// Pipe the current frame into ffmpeg
    pub fn save_frame(&mut self) {
        let stdin = &mut self.stdin;

        self.buffer.data.iter().for_each(|pixel| {
            let buf = <&P as Into<Rgb>>::into(pixel).vals;

            stdin
                .write_all(&buf)
                .expect("could not write to child stdin")
        });
    }

    /// Finish the video encoding operation.
    pub fn finish(mut self) {
        drop(self.stdin);
        self.child
            .wait()
            .expect("failed to wait for child process to exit");
    }
}

impl<P: Default> Frame<P> {
    fn new(resolution: (usize, usize)) -> Self {
        let (x, y) = resolution;
        let mut data = Vec::with_capacity(x * y);

        for _ in 0..x * y {
            data.push(P::default());
        }

        Self { data, resolution }
    }
}

impl<P> Frame<P> {
    fn verify_index(&self, index: (usize, usize)) {
        if index.0 >= self.resolution.0 {
            panic!(
                "frame index out of bounds: the x value is {} but the width is {}",
                index.0, self.resolution.0
            );
        } else if index.1 >= self.resolution.1 {
            panic!(
                "frame index out of bounds: the y value is {} but the height is {}",
                index.1, self.resolution.1
            );
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&P> {
        if x >= self.resolution.0 || y >= self.resolution.1 {
            None
        } else {
            Some(&self.data[x + self.resolution.0 * y])
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut P> {
        if x >= self.resolution.0 || y >= self.resolution.1 {
            None
        } else {
            Some(&mut self.data[x + self.resolution.0 * y])
        }
    }
}

impl<P> Index<(usize, usize)> for Frame<P> {
    type Output = P;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.verify_index(index);
        let (x, y) = index;

        self.get(x, y).unwrap()
    }
}

impl<P> IndexMut<(usize, usize)> for Frame<P> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.verify_index(index);
        let (x, y) = index;

        self.get_mut(x, y).unwrap()
    }
}
