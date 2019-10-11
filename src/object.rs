use crate::point::Point3;
use crate::vector::Vector3;
use crate::color::Rgb;

#[derive(Copy, Clone)]
pub enum Object {
	Sphere(Sphere),
	Plane(Plane),
}

impl Object {
	pub fn color(&self) -> Rgb {
		match self {
			Object::Sphere(object) => object.color(),
			Object::Plane(object) => object.color(),
		}
	}
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

	pub fn color(&self) -> Rgb {
		self.color
	}
}

#[derive(Copy, Clone)]
pub struct Plane {
	position: Point3,
	normal: Vector3,
	color: Rgb,
}

impl Plane {
	pub fn new(position: Point3, normal: Vector3, color: Rgb) -> Plane {
		let mut normal = normal;
		// make sure that the normal is a unit vector
		if normal.length() != 1.0 {
			normal.scale(1.0 / normal.length());
		}

		Plane {
			position: position,
			normal: normal,
			color: color,
		}
	}

	pub fn color(&self) -> Rgb {
		self.color
	}
}
