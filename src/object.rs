use crate::point::Point3;
use crate::color::Rgb;

#[derive(Copy, Clone)]
pub enum Object {
	Sphere(Sphere),
}

#[derive(Copy, Clone)]
pub struct Sphere {
	center: Point3,
	radius: f32,
	color: Rgb,
}

impl Sphere {
	pub fn new(center: Point3, radius: f32, color: Rgb) -> Sphere {
		Sphere {
			center: center,
			radius: radius,
			color: color,
		}
	}
}
