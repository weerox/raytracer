use image;

pub const RGB_BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
pub const RGB_WHITE: Rgb = Rgb { r: 255, g: 255, b: 255 };

pub const RGB_RED: Rgb = Rgb { r: 255, g: 0, b: 0 };
pub const RGB_GREEN: Rgb = Rgb { r: 0, g: 255, b: 0 };
pub const RGB_BLUE: Rgb = Rgb { r: 0, g: 0, b: 255 };

pub const RGB_CYAN: Rgb = Rgb { r: 0, g: 255, b: 255 };
pub const RGB_MAGENTA: Rgb = Rgb { r: 255, g: 0, b: 255 };
pub const RGB_YELLOW: Rgb = Rgb { r: 255, g: 255, b: 0 };

pub const RGB_PASTEL_BLUE: Rgb = Rgb { r: 174, g: 198, b: 207 };
pub const RGB_PASTEL_BROWN: Rgb = Rgb { r: 131, g: 105, b: 83 };
pub const RGB_PASTEL_GRAY: Rgb = Rgb { r: 207, g: 207, b: 196 };
pub const RGB_PASTEL_GREEN: Rgb = Rgb { r: 119, g: 221, b: 119 };
pub const RGB_PASTEL_MAGENTA: Rgb = Rgb { r: 244, g: 154, b: 194 };
pub const RGB_PASTEL_ORANGE: Rgb = Rgb { r: 255, g: 179, b: 71 };
pub const RGB_PASTEL_PINK: Rgb = Rgb { r: 255, g: 209, b: 220 };
pub const RGB_PASTEL_PURPLE: Rgb = Rgb { r: 179, g: 158, b: 181 };
pub const RGB_PASTEL_RED: Rgb = Rgb { r: 255, g: 105, b: 97 };
pub const RGB_PASTEL_VIOLET: Rgb = Rgb { r: 203, g: 153, b: 201 };
pub const RGB_PASTEL_YELLOW: Rgb = Rgb { r: 253, g: 253, b: 150 };

#[derive(Copy, Clone)]
pub struct Rgb {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Rgb {
	pub fn new(r: u8, g: u8, b: u8) -> Rgb {
		Rgb {
			r: r,
			g: g,
			b: b,
		}
	}

	pub fn to_pixel(&self) -> image::Rgb<u8> {
		return image::Rgb([self.r, self.g, self.b])
	}
}
