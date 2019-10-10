use image;

#[derive(Copy, Clone)]
pub struct Rgb {
	r: u8,
	g: u8,
	b: u8,
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
