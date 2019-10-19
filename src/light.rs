use crate::point::Point3;
use crate::vector::Vector3;

#[derive(Copy, Clone)]
pub enum Light {
	Directional(DirectionalLight),
	Point(PointLight),
}

#[derive(Copy, Clone)]
pub struct DirectionalLight {
	pub direction: Vector3,
}

impl DirectionalLight {
	pub fn new(direction: Vector3) -> DirectionalLight {
		DirectionalLight {
			direction: direction,
		}
	}
}

#[derive(Copy, Clone)]
pub struct PointLight {
	pub point: Point3,
}

impl PointLight {
	pub fn new(point: Point3) -> PointLight {
		PointLight {
			point: point,
		}
	}
}
