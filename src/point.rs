#[derive(Copy, Clone)]
pub struct Point3 {
	x: f32,
	y: f32,
	z: f32,
}

impl Point3 {
	pub fn new(x: f32, y: f32, z: f32) -> Point3 {
		Point3 {
			x: x,
			y: y,
			z: z,
		}
	}
}

pub struct Point2 {
	x: f32,
	y: f32,
}

impl Point2 {
	pub fn new(x: f32, y: f32) -> Point2 {
		Point2 {
			x: x,
			y: y,
		}
	}
}
