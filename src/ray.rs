use crate::point::Point3;
use crate::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Point3,
	pub direction: Vector3,
}

impl Ray {
	pub fn new(origin: Point3, direction: Vector3) -> Ray {
		Ray {
			origin: origin,
			direction: direction,
		}
	}
}
