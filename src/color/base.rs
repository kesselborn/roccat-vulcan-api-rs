#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
/// Caintains base color structure
use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};

/// RGB color representation
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ColorRgb {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRgb {
    /// Create a bew color from rgb values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// create [`ColorRgb`] form `[R, G, B]`
    pub const fn new_from_array(array: [u8; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }

    /// Get the red itensity
    pub const fn r(self) -> u8 {
        self.r
    }

    /// Get the green itensity
    pub const fn g(self) -> u8 {
        self.g
    }

    /// Get the blue itensity
    pub const fn b(self) -> u8 {
        self.b
    }

    /// Get the red itensity
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }

    /// Get the green itensity
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }

    /// Get the blue itensity
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }

    /// Retrn the color as a u32
    pub const fn into_u32(self) -> u32 {
        ((self.r as u32) << 16_u32) + ((self.g as u32) << 8_u32) + self.b as u32
    }

    /// Create the color form a u32
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_u32(c: u32) -> Self {
        Self::new((c >> 16) as u8, (c >> 8) as u8, c as u8)
    }
}

impl From<[u8; 3]> for ColorRgb {
    fn from(array: [u8; 3]) -> Self {
        ColorRgb::new_from_array(array)
    }
}

impl From<(u8, u8, u8)> for ColorRgb {
    fn from(tuple: (u8, u8, u8)) -> Self {
        let (r, g, b) = tuple;
        ColorRgb::new(r, g, b)
    }
}

impl From<u32> for ColorRgb {
    fn from(n: u32) -> Self {
        ColorRgb::from_u32(n)
    }
}

impl From<ColorRgb> for u32 {
    fn from(c: ColorRgb) -> Self {
        c.into_u32()
    }
}

impl From<ColorRgb> for (u8, u8, u8) {
    fn from(color: ColorRgb) -> Self {
        (color.r(), color.g(), color.b())
    }
}

impl Display for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r(), self.g(), self.b())
    }
}

impl Binary for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.into_u32())
    }
}

impl UpperHex for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.into_u32())
    }
}

impl LowerHex for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.into_u32())
    }
}

impl Octal for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.into_u32())
    }
}

/// Color with alpha parameter
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ColorRgba {
    color: ColorRgb,
    alpha: u8,
}

impl ColorRgba {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]

    /// Create a new RGBA color with the alpa value given by a [`f32`]
    pub fn new_from_float(r: u8, g: u8, b: u8, f: f32) -> Self {
        Self {
            color: ColorRgb::new(r, g, b),
            alpha: (f.max(0_f32).min(1_f32) * 255_f32).floor() as u8,
        }
    }

    /// Create a new RGBA color
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color: ColorRgb::new(r, g, b),
            alpha: a,
        }
    }

    /// Return the transparency value (0: tranparent, 255: opaque)
    pub const fn alpha(self) -> u8 {
        self.alpha
    }

    /// Return the row collor without the alpha
    pub const fn color(self) -> ColorRgb {
        self.color
    }

    /// applay the luminosity level to the value as it would be mixed with black
    #[allow(clippy::cast_possible_truncation)]
    const fn apply_lumninosity(value: u8, lum: u8) -> u8 {
        ((value as u16 * lum as u16) / 255_u16) as u8
    }

    /// Get the red value taking into account the luminosity
    pub const fn r(self) -> u8 {
        Self::apply_lumninosity(self.color().r(), self.alpha())
    }

    /// Get the green value taking into account the luminosity
    pub const fn g(self) -> u8 {
        Self::apply_lumninosity(self.color().g(), self.alpha())
    }

    /// Get the blue value taking into account the luminosity
    pub const fn b(self) -> u8 {
        Self::apply_lumninosity(self.color().b(), self.alpha())
    }

    /// Convert the color into a color RGB mixing it with black
    pub const fn into_color(self) -> ColorRgb {
        ColorRgb::new(self.r(), self.g(), self.b())
    }

    /// Convert the color to an u32
    pub const fn into_u32(self) -> u32 {
        (self.color().into_u32() << 4_u32) + (self.alpha() as u32)
    }

    /// Convert the color to an u32
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_u32(n: u32) -> Self {
        Self {
            color: ColorRgb::from_u32(n >> 4),
            alpha: n as u8,
        }
    }

    //TODO color add sub etc...Default
}

impl Display for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(r: {}, g: {}, b: {}, a: {})",
            self.color().r(),
            self.color().g(),
            self.color().b(),
            self.alpha()
        )
    }
}

impl Binary for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.into_u32())
    }
}

impl UpperHex for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.into_u32())
    }
}

impl LowerHex for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.into_u32())
    }
}

impl Octal for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.into_u32())
    }
}

impl From<ColorRgb> for ColorRgba {
    fn from(c: ColorRgb) -> Self {
        Self::new(c.r(), c.g(), c.b(), 255)
    }
}

impl From<ColorRgba> for ColorRgb {
    fn from(c: ColorRgba) -> Self {
        c.into_color()
    }
}

impl From<u32> for ColorRgba {
    fn from(n: u32) -> Self {
        ColorRgba::from_u32(n)
    }
}

impl From<ColorRgba> for u32 {
    fn from(c: ColorRgba) -> Self {
        c.into_u32()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color() {
        let ca = ColorRgba::new(30, 87, 70, 255);
        let c = ColorRgb::new(30, 87, 70);
        assert_eq!(c, ca.color());
        assert_eq!(c.r(), ca.r());

        let ca = ColorRgba::new(20, 0, 0, 128);
        assert_eq!(ca.r(), 10);
        assert_eq!(ca.b(), 0);
    }
}
