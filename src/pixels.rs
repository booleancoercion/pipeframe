//! Basic kinds of pixels, offering variation in the choice of frames.  
//! HSL and HSV algorithms adapted from [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

pub trait Pixel {
    fn to_rgb24(&self) -> [u8; 3];
}

pub struct Rgb {
    vals: [u8; 3],
}

pub struct Hsl {
    vals: [f64; 3],
}

pub struct Hsv {
    vals: [f64; 3],
}

impl Pixel for Rgb {
    fn to_rgb24(&self) -> [u8; 3] {
        self.vals
    }
}

impl Pixel for Hsl {
    #[allow(clippy::many_single_char_names)]
    fn to_rgb24(&self) -> [u8; 3] {
        let h = self.vals[0] * 360.;
        let s = self.vals[1];
        let l = self.vals[2];

        let a = s * l.min(1. - l);

        let f = |n: f64| {
            let k: f64 = (n + h / 30.).rem_euclid(12.);

            let val = l - a * (-1f64).max((k - 3.).min((9. - k).min(1.)));
            to_u8(val)
        };

        [f(0.), f(8.), f(4.)]
    }
}

impl Pixel for Hsv {
    #[allow(clippy::many_single_char_names)]
    fn to_rgb24(&self) -> [u8; 3] {
        let h = self.vals[0] * 360.;
        let s = self.vals[1];
        let v = self.vals[2];

        let f = |n: f64| {
            let k: f64 = (n + h / 60.).rem_euclid(6.);

            let val = v * (1. - s * (0f64).max(k.min((4. - k).min(1.))));

            to_u8(val)
        };

        [f(5.), f(3.), f(1.)]
    }
}

fn to_u8(fl: f64) -> u8 {
    (fl * 255.) as u8
}
