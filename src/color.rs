use palette::{FromColor, LinSrgb, LinSrgba, Srgb, Srgba, WithAlpha};

/// Color type with 32-bit float per channel.
/// Encoded as linear sRGB. Alpha is not premultiplied.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
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
        Self::BLACK
    }
}

impl Color {
    /// Converts a color from sRGB with nonlinear transfer function.
    pub const fn rgb8(r: u8, g: u8, b: u8) -> Self {
        use fast_srgb8::srgb8_to_f32;
        Self {
            red: srgb8_to_f32(r),
            green: srgb8_to_f32(g),
            blue: srgb8_to_f32(b),
            alpha: 1.0,
        }
    }

    /// Converts a color from sRGBA with nonlinear transfer function.
    pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            alpha: a as f32 / 255.0,
            ..Self::rgb8(r, g, b)
        }
    }

    /// Multiplies alpha by the given factor.
    #[must_use]
    pub fn with_alpha_factor(self, alpha: f32) -> Self {
        Self {
            alpha: self.alpha * alpha,
            ..self
        }
    }

    /// Premultiplies the alpha channel.
    #[must_use]
    pub fn premultiply(self) -> Self {
        Self {
            red: self.red * self.alpha,
            green: self.green * self.alpha,
            blue: self.blue * self.alpha,
            alpha: self.alpha,
        }
    }

    /// Performs linear interpolation.
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            red: lerp(self.red, other.red, t),
            green: lerp(self.green, other.green, t),
            blue: lerp(self.blue, other.blue, t),
            alpha: lerp(self.alpha, other.alpha, t),
        }
    }

    /// Parses a color from a string.
    ///
    /// Currently accepts CSS style hexadecimal colors of the forms #RGB, #RGBA,
    /// #RRGGBB, #RRGGBBAA or the name of an SVG color such as "aliceblue".
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        parse_color(s)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
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

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self {
            red: value[0],
            green: value[1],
            blue: value[2],
            alpha: value[3],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(value: Color) -> Self {
        [value.red, value.green, value.blue, value.alpha]
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self::rgb8(value[0], value[1], value[2])
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self::rgba8(value[0], value[1], value[2], value[3])
    }
}

/// Named SVG colors.
impl Color {
    /// Alice blue (240, 248, 255, 255)
    pub const ALICE_BLUE: Color = Color::rgb8(240, 248, 255);
    /// Antique white (250, 235, 215, 255)
    pub const ANTIQUE_WHITE: Color = Color::rgb8(250, 235, 215);
    /// Aqua (0, 255, 255, 255)
    pub const AQUA: Color = Color::rgb8(0, 255, 255);
    /// Aquamarine (127, 255, 212, 255)
    pub const AQUAMARINE: Color = Color::rgb8(127, 255, 212);
    /// Azure (240, 255, 255, 255)
    pub const AZURE: Color = Color::rgb8(240, 255, 255);
    /// Beige (245, 245, 220, 255)
    pub const BEIGE: Color = Color::rgb8(245, 245, 220);
    /// Bisque (255, 228, 196, 255)
    pub const BISQUE: Color = Color::rgb8(255, 228, 196);
    /// Black (0, 0, 0, 255)
    pub const BLACK: Color = Color::rgb8(0, 0, 0);
    /// Blanched almond (255, 235, 205, 255)
    pub const BLANCHED_ALMOND: Color = Color::rgb8(255, 235, 205);
    /// Blue (0, 0, 255, 255)
    pub const BLUE: Color = Color::rgb8(0, 0, 255);
    /// Blue violet (138, 43, 226, 255)
    pub const BLUE_VIOLET: Color = Color::rgb8(138, 43, 226);
    /// Brown (165, 42, 42, 255)
    pub const BROWN: Color = Color::rgb8(165, 42, 42);
    /// Burlywood (222, 184, 135, 255)
    pub const BURLYWOOD: Color = Color::rgb8(222, 184, 135);
    /// Cadet blue (95, 158, 160, 255)
    pub const CADET_BLUE: Color = Color::rgb8(95, 158, 160);
    /// Chartreuse (127, 255, 0, 255)
    pub const CHARTREUSE: Color = Color::rgb8(127, 255, 0);
    /// Chocolate (210, 105, 30, 255)
    pub const CHOCOLATE: Color = Color::rgb8(210, 105, 30);
    /// Coral (255, 127, 80, 255)
    pub const CORAL: Color = Color::rgb8(255, 127, 80);
    /// Cornflower blue (100, 149, 237, 255)
    pub const CORNFLOWER_BLUE: Color = Color::rgb8(100, 149, 237);
    /// Cornsilk (255, 248, 220, 255)
    pub const CORNSILK: Color = Color::rgb8(255, 248, 220);
    /// Crimson (220, 20, 60, 255)
    pub const CRIMSON: Color = Color::rgb8(220, 20, 60);
    /// Cyan (0, 255, 255, 255)
    pub const CYAN: Color = Color::rgb8(0, 255, 255);
    /// Dark blue (0, 0, 139, 255)
    pub const DARK_BLUE: Color = Color::rgb8(0, 0, 139);
    /// Dark cyan (0, 139, 139, 255)
    pub const DARK_CYAN: Color = Color::rgb8(0, 139, 139);
    /// Dark goldenrod (184, 134, 11, 255)
    pub const DARK_GOLDENROD: Color = Color::rgb8(184, 134, 11);
    /// Dark gray (169, 169, 169, 255)
    pub const DARK_GRAY: Color = Color::rgb8(169, 169, 169);
    /// Dark green (0, 100, 0, 255)
    pub const DARK_GREEN: Color = Color::rgb8(0, 100, 0);
    /// Dark khaki (189, 183, 107, 255)
    pub const DARK_KHAKI: Color = Color::rgb8(189, 183, 107);
    /// Dark magenta (139, 0, 139, 255)
    pub const DARK_MAGENTA: Color = Color::rgb8(139, 0, 139);
    /// Dark olive green (85, 107, 47, 255)
    pub const DARK_OLIVE_GREEN: Color = Color::rgb8(85, 107, 47);
    /// Dark orange (255, 140, 0, 255)
    pub const DARK_ORANGE: Color = Color::rgb8(255, 140, 0);
    /// Dark orchid (153, 50, 204, 255)
    pub const DARK_ORCHID: Color = Color::rgb8(153, 50, 204);
    /// Dark red (139, 0, 0, 255)
    pub const DARK_RED: Color = Color::rgb8(139, 0, 0);
    /// Dark salmon (233, 150, 122, 255)
    pub const DARK_SALMON: Color = Color::rgb8(233, 150, 122);
    /// Dark sea green (143, 188, 143, 255)
    pub const DARK_SEA_GREEN: Color = Color::rgb8(143, 188, 143);
    /// Dark slate blue (72, 61, 139, 255)
    pub const DARK_SLATE_BLUE: Color = Color::rgb8(72, 61, 139);
    /// Dark slate gray (47, 79, 79, 255)
    pub const DARK_SLATE_GRAY: Color = Color::rgb8(47, 79, 79);
    /// Dark turquoise (0, 206, 209, 255)
    pub const DARK_TURQUOISE: Color = Color::rgb8(0, 206, 209);
    /// Dark violet (148, 0, 211, 255)
    pub const DARK_VIOLET: Color = Color::rgb8(148, 0, 211);
    /// Deep pink (255, 20, 147, 255)
    pub const DEEP_PINK: Color = Color::rgb8(255, 20, 147);
    /// Deep sky blue (0, 191, 255, 255)
    pub const DEEP_SKY_BLUE: Color = Color::rgb8(0, 191, 255);
    /// Dim gray (105, 105, 105, 255)
    pub const DIM_GRAY: Color = Color::rgb8(105, 105, 105);
    /// Dodger blue (30, 144, 255, 255)
    pub const DODGER_BLUE: Color = Color::rgb8(30, 144, 255);
    /// Firebrick (178, 34, 34, 255)
    pub const FIREBRICK: Color = Color::rgb8(178, 34, 34);
    /// Floral white (255, 250, 240, 255)
    pub const FLORAL_WHITE: Color = Color::rgb8(255, 250, 240);
    /// Forest green (34, 139, 34, 255)
    pub const FOREST_GREEN: Color = Color::rgb8(34, 139, 34);
    /// Fuchsia (255, 0, 255, 255)
    pub const FUCHSIA: Color = Color::rgb8(255, 0, 255);
    /// Gainsboro (220, 220, 220, 255)
    pub const GAINSBORO: Color = Color::rgb8(220, 220, 220);
    /// Ghost white (248, 248, 255, 255)
    pub const GHOST_WHITE: Color = Color::rgb8(248, 248, 255);
    /// Gold (255, 215, 0, 255)
    pub const GOLD: Color = Color::rgb8(255, 215, 0);
    /// Goldenrod (218, 165, 32, 255)
    pub const GOLDENROD: Color = Color::rgb8(218, 165, 32);
    /// Gray (128, 128, 128, 255)
    pub const GRAY: Color = Color::rgb8(128, 128, 128);
    /// Green (0, 128, 0, 255)
    pub const GREEN: Color = Color::rgb8(0, 128, 0);
    /// Green yellow (173, 255, 47, 255)
    pub const GREEN_YELLOW: Color = Color::rgb8(173, 255, 47);
    /// Honeydew (240, 255, 240, 255)
    pub const HONEYDEW: Color = Color::rgb8(240, 255, 240);
    /// Hot pink (255, 105, 180, 255)
    pub const HOT_PINK: Color = Color::rgb8(255, 105, 180);
    /// Indian red (205, 92, 92, 255)
    pub const INDIAN_RED: Color = Color::rgb8(205, 92, 92);
    /// Indigo (75, 0, 130, 255)
    pub const INDIGO: Color = Color::rgb8(75, 0, 130);
    /// Ivory (255, 255, 240, 255)
    pub const IVORY: Color = Color::rgb8(255, 255, 240);
    /// Khaki (240, 230, 140, 255)
    pub const KHAKI: Color = Color::rgb8(240, 230, 140);
    /// Lavender (230, 230, 250, 255)
    pub const LAVENDER: Color = Color::rgb8(230, 230, 250);
    /// Lavender blush (255, 240, 245, 255)
    pub const LAVENDER_BLUSH: Color = Color::rgb8(255, 240, 245);
    /// Lawn green (124, 252, 0, 255)
    pub const LAWN_GREEN: Color = Color::rgb8(124, 252, 0);
    /// Lemon chiffon (255, 250, 205, 255)
    pub const LEMON_CHIFFON: Color = Color::rgb8(255, 250, 205);
    /// Light blue (173, 216, 230, 255)
    pub const LIGHT_BLUE: Color = Color::rgb8(173, 216, 230);
    /// Light coral (240, 128, 128, 255)
    pub const LIGHT_CORAL: Color = Color::rgb8(240, 128, 128);
    /// Light cyan (224, 255, 255, 255)
    pub const LIGHT_CYAN: Color = Color::rgb8(224, 255, 255);
    /// Light goldenrod yellow (250, 250, 210, 255)
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::rgb8(250, 250, 210);
    /// Light gray (211, 211, 211, 255)
    pub const LIGHT_GRAY: Color = Color::rgb8(211, 211, 211);
    /// Light green (144, 238, 144, 255)
    pub const LIGHT_GREEN: Color = Color::rgb8(144, 238, 144);
    /// Light pink (255, 182, 193, 255)
    pub const LIGHT_PINK: Color = Color::rgb8(255, 182, 193);
    /// Light salmon (255, 160, 122, 255)
    pub const LIGHT_SALMON: Color = Color::rgb8(255, 160, 122);
    /// Light sea green (32, 178, 170, 255)
    pub const LIGHT_SEA_GREEN: Color = Color::rgb8(32, 178, 170);
    /// Light sky blue (135, 206, 250, 255)
    pub const LIGHT_SKY_BLUE: Color = Color::rgb8(135, 206, 250);
    /// Light slate gray (119, 136, 153, 255)
    pub const LIGHT_SLATE_GRAY: Color = Color::rgb8(119, 136, 153);
    /// Light steel blue (176, 196, 222, 255)
    pub const LIGHT_STEEL_BLUE: Color = Color::rgb8(176, 196, 222);
    /// Light yellow (255, 255, 224, 255)
    pub const LIGHT_YELLOW: Color = Color::rgb8(255, 255, 224);
    /// Lime (0, 255, 0, 255)
    pub const LIME: Color = Color::rgb8(0, 255, 0);
    /// Lime green (50, 205, 50, 255)
    pub const LIME_GREEN: Color = Color::rgb8(50, 205, 50);
    /// Linen (250, 240, 230, 255)
    pub const LINEN: Color = Color::rgb8(250, 240, 230);
    /// Magenta (255, 0, 255, 255)
    pub const MAGENTA: Color = Color::rgb8(255, 0, 255);
    /// Maroon (128, 0, 0, 255)
    pub const MAROON: Color = Color::rgb8(128, 0, 0);
    /// Medium aquamarine (102, 205, 170, 255)
    pub const MEDIUM_AQUAMARINE: Color = Color::rgb8(102, 205, 170);
    /// Medium blue (0, 0, 205, 255)
    pub const MEDIUM_BLUE: Color = Color::rgb8(0, 0, 205);
    /// Medium orchid (186, 85, 211, 255)
    pub const MEDIUM_ORCHID: Color = Color::rgb8(186, 85, 211);
    /// Medium purple (147, 112, 219, 255)
    pub const MEDIUM_PURPLE: Color = Color::rgb8(147, 112, 219);
    /// Medium sea green (60, 179, 113, 255)
    pub const MEDIUM_SEA_GREEN: Color = Color::rgb8(60, 179, 113);
    /// Medium slate blue (123, 104, 238, 255)
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgb8(123, 104, 238);
    /// Medium spring green (0, 250, 154, 255)
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgb8(0, 250, 154);
    /// Medium turquoise (72, 209, 204, 255)
    pub const MEDIUM_TURQUOISE: Color = Color::rgb8(72, 209, 204);
    /// Medium violet red (199, 21, 133, 255)
    pub const MEDIUM_VIOLET_RED: Color = Color::rgb8(199, 21, 133);
    /// Midnight blue (25, 25, 112, 255)
    pub const MIDNIGHT_BLUE: Color = Color::rgb8(25, 25, 112);
    /// Mint cream (245, 255, 250, 255)
    pub const MINT_CREAM: Color = Color::rgb8(245, 255, 250);
    /// Misty rose (255, 228, 225, 255)
    pub const MISTY_ROSE: Color = Color::rgb8(255, 228, 225);
    /// Moccasin (255, 228, 181, 255)
    pub const MOCCASIN: Color = Color::rgb8(255, 228, 181);
    /// Navajo white (255, 222, 173, 255)
    pub const NAVAJO_WHITE: Color = Color::rgb8(255, 222, 173);
    /// Navy (0, 0, 128, 255)
    pub const NAVY: Color = Color::rgb8(0, 0, 128);
    /// Old lace (253, 245, 230, 255)
    pub const OLD_LACE: Color = Color::rgb8(253, 245, 230);
    /// Olive (128, 128, 0, 255)
    pub const OLIVE: Color = Color::rgb8(128, 128, 0);
    /// Olive drab (107, 142, 35, 255)
    pub const OLIVE_DRAB: Color = Color::rgb8(107, 142, 35);
    /// Orange (255, 165, 0, 255)
    pub const ORANGE: Color = Color::rgb8(255, 165, 0);
    /// Orange red (255, 69, 0, 255)
    pub const ORANGE_RED: Color = Color::rgb8(255, 69, 0);
    /// Orchid (218, 112, 214, 255)
    pub const ORCHID: Color = Color::rgb8(218, 112, 214);
    /// Pale goldenrod (238, 232, 170, 255)
    pub const PALE_GOLDENROD: Color = Color::rgb8(238, 232, 170);
    /// Pale green (152, 251, 152, 255)
    pub const PALE_GREEN: Color = Color::rgb8(152, 251, 152);
    /// Pale turquoise (175, 238, 238, 255)
    pub const PALE_TURQUOISE: Color = Color::rgb8(175, 238, 238);
    /// Pale violet red (219, 112, 147, 255)
    pub const PALE_VIOLET_RED: Color = Color::rgb8(219, 112, 147);
    /// Papaya whip (255, 239, 213, 255)
    pub const PAPAYA_WHIP: Color = Color::rgb8(255, 239, 213);
    /// Peach puff (255, 218, 185, 255)
    pub const PEACH_PUFF: Color = Color::rgb8(255, 218, 185);
    /// Peru (205, 133, 63, 255)
    pub const PERU: Color = Color::rgb8(205, 133, 63);
    /// Pink (255, 192, 203, 255)
    pub const PINK: Color = Color::rgb8(255, 192, 203);
    /// Plum (221, 160, 221, 255)
    pub const PLUM: Color = Color::rgb8(221, 160, 221);
    /// Powder blue (176, 224, 230, 255)
    pub const POWDER_BLUE: Color = Color::rgb8(176, 224, 230);
    /// Purple (128, 0, 128, 255)
    pub const PURPLE: Color = Color::rgb8(128, 0, 128);
    /// Rebecca purple (102, 51, 153, 255)
    pub const REBECCA_PURPLE: Color = Color::rgb8(102, 51, 153);
    /// Red (255, 0, 0, 255)
    pub const RED: Color = Color::rgb8(255, 0, 0);
    /// Rosy brown (188, 143, 143, 255)
    pub const ROSY_BROWN: Color = Color::rgb8(188, 143, 143);
    /// Royal blue (65, 105, 225, 255)
    pub const ROYAL_BLUE: Color = Color::rgb8(65, 105, 225);
    /// Saddle brown (139, 69, 19, 255)
    pub const SADDLE_BROWN: Color = Color::rgb8(139, 69, 19);
    /// Salmon (250, 128, 114, 255)
    pub const SALMON: Color = Color::rgb8(250, 128, 114);
    /// Sandy brown (244, 164, 96, 255)
    pub const SANDY_BROWN: Color = Color::rgb8(244, 164, 96);
    /// Sea green (46, 139, 87, 255)
    pub const SEA_GREEN: Color = Color::rgb8(46, 139, 87);
    /// Seashell (255, 245, 238, 255)
    pub const SEASHELL: Color = Color::rgb8(255, 245, 238);
    /// Sienna (160, 82, 45, 255)
    pub const SIENNA: Color = Color::rgb8(160, 82, 45);
    /// Silver (192, 192, 192, 255)
    pub const SILVER: Color = Color::rgb8(192, 192, 192);
    /// Sky blue (135, 206, 235, 255)
    pub const SKY_BLUE: Color = Color::rgb8(135, 206, 235);
    /// Slate blue (106, 90, 205, 255)
    pub const SLATE_BLUE: Color = Color::rgb8(106, 90, 205);
    /// Slate gray (112, 128, 144, 255)
    pub const SLATE_GRAY: Color = Color::rgb8(112, 128, 144);
    /// Snow (255, 250, 250, 255)
    pub const SNOW: Color = Color::rgb8(255, 250, 250);
    /// Spring green (0, 255, 127, 255)
    pub const SPRING_GREEN: Color = Color::rgb8(0, 255, 127);
    /// Steel blue (70, 130, 180, 255)
    pub const STEEL_BLUE: Color = Color::rgb8(70, 130, 180);
    /// Tan (210, 180, 140, 255)
    pub const TAN: Color = Color::rgb8(210, 180, 140);
    /// Teal (0, 128, 128, 255)
    pub const TEAL: Color = Color::rgb8(0, 128, 128);
    /// Thistle (216, 191, 216, 255)
    pub const THISTLE: Color = Color::rgb8(216, 191, 216);
    /// Tomato (255, 99, 71, 255)
    pub const TOMATO: Color = Color::rgb8(255, 99, 71);
    /// Transparent (0, 0, 0, 0)
    pub const TRANSPARENT: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };
    /// Turquoise (64, 224, 208, 255)
    pub const TURQUOISE: Color = Color::rgb8(64, 224, 208);
    /// Violet (238, 130, 238, 255)
    pub const VIOLET: Color = Color::rgb8(238, 130, 238);
    /// Wheat (245, 222, 179, 255)
    pub const WHEAT: Color = Color::rgb8(245, 222, 179);
    /// White (255, 255, 255, 255)
    pub const WHITE: Color = Color::rgb8(255, 255, 255);
    /// White smoke (245, 245, 245, 255)
    pub const WHITE_SMOKE: Color = Color::rgb8(245, 245, 245);
    /// Yellow (255, 255, 0, 255)
    pub const YELLOW: Color = Color::rgb8(255, 255, 0);
    /// Yellow green (154, 205, 50, 255)
    pub const YELLOW_GREEN: Color = Color::rgb8(154, 205, 50);
}

fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if let Some(stripped) = s.strip_prefix('#') {
        Some(color_from_4bit_hex(get_4bit_hex_channels(stripped)?))
    } else {
        Some(match s {
            "aliceblue" => Color::ALICE_BLUE,
            "antiquewhite" => Color::ANTIQUE_WHITE,
            "aqua" => Color::AQUA,
            "aquamarine" => Color::AQUAMARINE,
            "azure" => Color::AZURE,
            "beige" => Color::BEIGE,
            "bisque" => Color::BISQUE,
            "black" => Color::BLACK,
            "blanchedalmond" => Color::BLANCHED_ALMOND,
            "blue" => Color::BLUE,
            "blueviolet" => Color::BLUE_VIOLET,
            "brown" => Color::BROWN,
            "burlywood" => Color::BURLYWOOD,
            "cadetblue" => Color::CADET_BLUE,
            "chartreuse" => Color::CHARTREUSE,
            "chocolate" => Color::CHOCOLATE,
            "coral" => Color::CORAL,
            "cornflowerblue" => Color::CORNFLOWER_BLUE,
            "cornsilk" => Color::CORNSILK,
            "crimson" => Color::CRIMSON,
            "cyan" => Color::CYAN,
            "darkblue" => Color::DARK_BLUE,
            "darkcyan" => Color::DARK_CYAN,
            "darkgoldenrod" => Color::DARK_GOLDENROD,
            "darkgray" => Color::DARK_GRAY,
            "darkgreen" => Color::DARK_GREEN,
            "darkkhaki" => Color::DARK_KHAKI,
            "darkmagenta" => Color::DARK_MAGENTA,
            "darkolivegreen" => Color::DARK_OLIVE_GREEN,
            "darkorange" => Color::DARK_ORANGE,
            "darkorchid" => Color::DARK_ORCHID,
            "darkred" => Color::DARK_RED,
            "darksalmon" => Color::DARK_SALMON,
            "darkseagreen" => Color::DARK_SEA_GREEN,
            "darkslateblue" => Color::DARK_SLATE_BLUE,
            "darkslategray" => Color::DARK_SLATE_GRAY,
            "darkturquoise" => Color::DARK_TURQUOISE,
            "darkviolet" => Color::DARK_VIOLET,
            "deeppink" => Color::DEEP_PINK,
            "deepskyblue" => Color::DEEP_SKY_BLUE,
            "dimgray" => Color::DIM_GRAY,
            "dodgerblue" => Color::DODGER_BLUE,
            "firebrick" => Color::FIREBRICK,
            "floralwhite" => Color::FLORAL_WHITE,
            "forestgreen" => Color::FOREST_GREEN,
            "fuchsia" => Color::FUCHSIA,
            "gainsboro" => Color::GAINSBORO,
            "ghostwhite" => Color::GHOST_WHITE,
            "gold" => Color::GOLD,
            "goldenrod" => Color::GOLDENROD,
            "gray" => Color::GRAY,
            "green" => Color::GREEN,
            "greenyellow" => Color::GREEN_YELLOW,
            "honeydew" => Color::HONEYDEW,
            "hotpink" => Color::HOT_PINK,
            "indianred" => Color::INDIAN_RED,
            "indigo" => Color::INDIGO,
            "ivory" => Color::IVORY,
            "khaki" => Color::KHAKI,
            "lavender" => Color::LAVENDER,
            "lavenderblush" => Color::LAVENDER_BLUSH,
            "lawngreen" => Color::LAWN_GREEN,
            "lemonchiffon" => Color::LEMON_CHIFFON,
            "lightblue" => Color::LIGHT_BLUE,
            "lightcoral" => Color::LIGHT_CORAL,
            "lightcyan" => Color::LIGHT_CYAN,
            "lightgoldenrodyellow" => Color::LIGHT_GOLDENROD_YELLOW,
            "lightgray" => Color::LIGHT_GRAY,
            "lightgreen" => Color::LIGHT_GREEN,
            "lightpink" => Color::LIGHT_PINK,
            "lightsalmon" => Color::LIGHT_SALMON,
            "lightseagreen" => Color::LIGHT_SEA_GREEN,
            "lightskyblue" => Color::LIGHT_SKY_BLUE,
            "lightslategray" => Color::LIGHT_SLATE_GRAY,
            "lightsteelblue" => Color::LIGHT_STEEL_BLUE,
            "lightyellow" => Color::LIGHT_YELLOW,
            "lime" => Color::LIME,
            "limegreen" => Color::LIME_GREEN,
            "linen" => Color::LINEN,
            "magenta" => Color::MAGENTA,
            "maroon" => Color::MAROON,
            "mediumaquamarine" => Color::MEDIUM_AQUAMARINE,
            "mediumblue" => Color::MEDIUM_BLUE,
            "mediumorchid" => Color::MEDIUM_ORCHID,
            "mediumpurple" => Color::MEDIUM_PURPLE,
            "mediumseagreen" => Color::MEDIUM_SEA_GREEN,
            "mediumslateblue" => Color::MEDIUM_SLATE_BLUE,
            "mediumspringgreen" => Color::MEDIUM_SPRING_GREEN,
            "mediumturquoise" => Color::MEDIUM_TURQUOISE,
            "mediumvioletred" => Color::MEDIUM_VIOLET_RED,
            "midnightblue" => Color::MIDNIGHT_BLUE,
            "mintcream" => Color::MINT_CREAM,
            "mistyrose" => Color::MISTY_ROSE,
            "moccasin" => Color::MOCCASIN,
            "navajowhite" => Color::NAVAJO_WHITE,
            "navy" => Color::NAVY,
            "oldlace" => Color::OLD_LACE,
            "olive" => Color::OLIVE,
            "olivedrab" => Color::OLIVE_DRAB,
            "orange" => Color::ORANGE,
            "orangered" => Color::ORANGE_RED,
            "orchid" => Color::ORCHID,
            "palegoldenrod" => Color::PALE_GOLDENROD,
            "palegreen" => Color::PALE_GREEN,
            "paleturquoise" => Color::PALE_TURQUOISE,
            "palevioletred" => Color::PALE_VIOLET_RED,
            "papayawhip" => Color::PAPAYA_WHIP,
            "peachpuff" => Color::PEACH_PUFF,
            "peru" => Color::PERU,
            "pink" => Color::PINK,
            "plum" => Color::PLUM,
            "powderblue" => Color::POWDER_BLUE,
            "purple" => Color::PURPLE,
            "rebeccapurple" => Color::REBECCA_PURPLE,
            "red" => Color::RED,
            "rosybrown" => Color::ROSY_BROWN,
            "royalblue" => Color::ROYAL_BLUE,
            "saddlebrown" => Color::SADDLE_BROWN,
            "salmon" => Color::SALMON,
            "sandybrown" => Color::SANDY_BROWN,
            "seagreen" => Color::SEA_GREEN,
            "seashell" => Color::SEASHELL,
            "sienna" => Color::SIENNA,
            "silver" => Color::SILVER,
            "skyblue" => Color::SKY_BLUE,
            "slateblue" => Color::SLATE_BLUE,
            "slategray" => Color::SLATE_GRAY,
            "snow" => Color::SNOW,
            "springgreen" => Color::SPRING_GREEN,
            "steelblue" => Color::STEEL_BLUE,
            "tan" => Color::TAN,
            "teal" => Color::TEAL,
            "thistle" => Color::THISTLE,
            "tomato" => Color::TOMATO,
            "transparent" => Color::TRANSPARENT,
            "turquoise" => Color::TURQUOISE,
            "violet" => Color::VIOLET,
            "wheat" => Color::WHEAT,
            "white" => Color::WHITE,
            "whitesmoke" => Color::WHITE_SMOKE,
            "yellow" => Color::YELLOW,
            "yellowgreen" => Color::YELLOW_GREEN,
            _ => return None,
        })
    }
}

// The following hex color parsing code taken from piet:

const fn get_4bit_hex_channels(hex_str: &str) -> Option<[u8; 8]> {
    let mut four_bit_channels = match hex_str.as_bytes() {
        &[b'#', r, g, b] | &[r, g, b] => [r, r, g, g, b, b, b'f', b'f'],
        &[b'#', r, g, b, a] | &[r, g, b, a] => [r, r, g, g, b, b, a, a],
        &[b'#', r0, r1, g0, g1, b0, b1] | &[r0, r1, g0, g1, b0, b1] => {
            [r0, r1, g0, g1, b0, b1, b'f', b'f']
        }
        &[b'#', r0, r1, g0, g1, b0, b1, a0, a1] | &[r0, r1, g0, g1, b0, b1, a0, a1] => {
            [r0, r1, g0, g1, b0, b1, a0, a1]
        }
        _ => return None,
    };

    // convert to hex in-place
    // this is written without a for loop to satisfy `const`
    let mut i = 0;
    while i < four_bit_channels.len() {
        let ascii = four_bit_channels[i];
        let as_hex = match hex_from_ascii_byte(ascii) {
            Ok(hex) => hex,
            Err(_) => return None,
        };
        four_bit_channels[i] = as_hex;
        i += 1;
    }
    Some(four_bit_channels)
}

fn color_from_4bit_hex(components: [u8; 8]) -> Color {
    let [r0, r1, g0, g1, b0, b1, a0, a1] = components;
    Color::rgba8(r0 << 4 | r1, g0 << 4 | g1, b0 << 4 | b1, a0 << 4 | a1)
}

const fn hex_from_ascii_byte(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        _ => Err(b),
    }
}
