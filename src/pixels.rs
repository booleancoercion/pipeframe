//! Basic kinds of pixels, offering variation in the choice of frames.  
//! HSL and HSV algorithms adapted from [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).
//!
//! Every type of pixel must satisfy the trait bound `Into<Rgb>`, where `Rgb` is the struct in this module.

/// A simple RGB pixel.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rgb {
    vals: [u8; 3],
}

impl Rgb {
    /// Construct a new RGB pixel from a triplet of bytes.
    pub fn bytes(bytes: [u8; 3]) -> Self {
        Self { vals: bytes }
    }

    /// Construct a new RGB pixel from a triplet of floats;
    /// each float value should be between 0.0 and 1.0 (where 0.0 -> 0, 1.0 -> 255).  
    /// The inputs are clamped in order to satisfy this.
    pub fn floats(floats: [f64; 3]) -> Self {
        let vals = [to_u8(floats[0]), to_u8(floats[1]), to_u8(floats[2])];

        Self { vals }
    }
}

/// An HSL (Hue, Saturation, Lightness) pixel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    vals: [f64; 3],
}

impl Hsl {
    /// Construct a new HSL pixel from a triplet of integers (u16s).
    /// The integers should satisfy the following bounds:
    ///
    /// 0 <= H <= 360  
    /// 0 <= S <= 100  
    /// 0 <= L <= 100  
    /// In that order; that is, H = ints[0] and so on. The inputs are clamped to fit in this range.
    pub fn ints(ints: [u16; 3]) -> Self {
        let h = ints[0].clamp(0, 360) as f64;
        let s = ints[1].clamp(0, 100) as f64;
        let l = ints[2].clamp(0, 100) as f64;

        Self {
            vals: [h / 360., s / 100., l / 100.],
        }
    }

    /// Construct a new HSL pixel from a triplet of floats.
    /// The floats should all be between 0.0 and 1.0, and will be clamped into that range.
    pub fn floats(floats: [f64; 3]) -> Self {
        let h = floats[0].clamp(0.0, 1.0);
        let s = floats[1].clamp(0.0, 1.0);
        let l = floats[2].clamp(0.0, 1.0);

        Self { vals: [h, s, l] }
    }
}

/// An HSV (Hue, Saturation, Value/Brightness) pixel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    vals: [f64; 3],
}

impl Hsv {
    /// Construct a new HSV pixel from a triplet of integers (u16s).
    /// The integers should satisfy the following bounds:
    ///
    /// 0 <= H <= 360  
    /// 0 <= S <= 100  
    /// 0 <= V <= 100  
    /// In that order; that is, H = ints[0] and so on. The inputs are clamped to fit in this range.
    pub fn ints(ints: [u16; 3]) -> Self {
        // same impl
        Self {
            vals: Hsl::ints(ints).vals,
        }
    }

    /// Construct a new HSV pixel from a triplet of floats.
    /// The floats should all be between 0.0 and 1.0, and will be clamped into that range.
    pub fn floats(floats: [f64; 3]) -> Self {
        // same impl
        Self {
            vals: Hsl::floats(floats).vals,
        }
    }
}

impl From<Hsl> for Rgb {
    #[allow(clippy::many_single_char_names)]
    fn from(other: Hsl) -> Self {
        let h = other.vals[0] * 360.;
        let s = other.vals[1];
        let l = other.vals[2];

        let a = s * l.min(1. - l);

        let f = |n: f64| {
            let k: f64 = (n + h / 30.).rem_euclid(12.);

            let val = l - a * (-1f64).max((k - 3.).min((9. - k).min(1.)));
            to_u8(val)
        };

        Self {
            vals: [f(0.), f(8.), f(4.)],
        }
    }
}

impl From<Hsv> for Rgb {
    #[allow(clippy::many_single_char_names)]
    fn from(other: Hsv) -> Self {
        let h = other.vals[0] * 360.;
        let s = other.vals[1];
        let v = other.vals[2];

        let f = |n: f64| {
            let k: f64 = (n + h / 60.).rem_euclid(6.);

            let val = v * (1. - s * (0f64).max(k.min((4. - k).min(1.))));

            to_u8(val)
        };

        Self {
            vals: [f(5.), f(3.), f(1.)],
        }
    }
}

fn to_u8(fl: f64) -> u8 {
    (fl * 254.99).round().clamp(0., 255.) as u8
}
