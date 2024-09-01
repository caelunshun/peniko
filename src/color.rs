use palette::{FromColor, LinSrgb, LinSrgba, Srgb, Srgba, WithAlpha};

/// Color type with 32-bit float per channel.
/// Encoded as linear sRGB. Alpha is not premultiplied.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// Red component. Linear.
    pub red: f32,
    /// Green component. Linear.
    pub green: f32,
    /// Blue component. Linear.
    pub blue: f32,
    /// Alpha component. Linear.
    pub alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

impl Color {
    /// White.
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    };
    /// Black.
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };
    /// Transparent black.
    pub const TRANSPARENT: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };

    /// Converts a color from sRGB with nonlinear transfer function.
    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Srgb::<u8>::new(r, g, b).into()
    }

    /// Converts a color from sRGBA with nonlinear transfer function.
    pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Srgba::<u8>::new(r, g, b, a).into()
    }

    /// Multiplies alpha by the given factor.
    #[must_use]
    pub fn with_alpha_factor(self, alpha: f32) -> Self {
        Self {
            alpha: self.alpha * alpha,
            ..self
        }
    }
}

impl From<LinSrgba> for Color {
    fn from(value: LinSrgba) -> Self {
        Self {
            red: value.red,
            green: value.green,
            blue: value.blue,
            alpha: value.alpha,
        }
    }
}

impl From<LinSrgb> for Color {
    fn from(value: LinSrgb) -> Self {
        Self::from(value.with_alpha(1.0))
    }
}

impl From<Srgba> for Color {
    fn from(value: Srgba) -> Self {
        Self::from(LinSrgba::from_color(value))
    }
}

impl From<Srgb> for Color {
    fn from(value: Srgb) -> Self {
        Self::from(LinSrgba::from_color(value.with_alpha(1.0)))
    }
}

impl From<Srgba<u8>> for Color {
    fn from(value: Srgba<u8>) -> Self {
        Self::from(value.into_linear::<f32, f32>().into_format())
    }
}

impl From<Srgb<u8>> for Color {
    fn from(value: Srgb<u8>) -> Self {
        Self::from(value.into_linear::<f32>().with_alpha(1.0).into_format())
    }
}
