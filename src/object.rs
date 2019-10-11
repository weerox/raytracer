use crate::point::Point3;
use crate::vector::Vector3;
use crate::ray::Ray;
use crate::color::Rgb;

#[derive(Copy, Clone)]
pub enum Object {
	Sphere(Sphere),
	Plane(Plane),
}

impl Object {
	pub fn intersects(&self, ray: Ray) -> Option<f32> {
		match self {
			Object::Sphere(object) => object.intersects(ray),
			Object::Plane(object) => object.intersects(ray),
		}
	}

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

	pub fn intersects(&self, ray: Ray) -> Option<f32> {
		let mut ray = ray;
		// make sure that the ray is a unit vector
		if ray.direction.length() != 1.0 {
			ray.direction.scale(1.0 / ray.direction.length());
		}

		let mut origin_to_center = ray.origin.as_vector();
		origin_to_center.subtract(self.center.as_vector());

		// formula from en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

		let a = ray.direction.dot(ray.direction);
		let b = 2.0 * ray.direction.dot(origin_to_center);
		let c = origin_to_center.dot(origin_to_center) - self.radius * self.radius;

		// solve for the distance using pq-formula
		// the discriminant describes the number of intersections

		let discriminant = b * b - 4.0 * a * c;

		if discriminant == 0.0 {
			// one solution exists
			let d = -b / (2.0 * a);

			if d >= 0.0 {
				return Some(d);
			}
		} else if discriminant > 0.0 {
			let d = -b / (2.0 * a);
			let d1 = d + discriminant.sqrt();
			let d2 = d - discriminant.sqrt();

			if d1 > 0.0 && d2 > 0.0 {
				return Some(d1.min(d2));
			} else if d1 <= 0.0 && d2 <= 0.0 {
				return None;
			} else {
				return Some(d1.max(d2));
			}
		}

		return None;
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

	pub fn intersects(&self, ray: Ray) -> Option<f32> {
		let mut ray = ray;
		// make sure that the ray is a unit vector
		if ray.direction.length() != 1.0 {
			ray.direction.scale(1.0 / ray.direction.length());
		}

		let numerator = self.position.as_vector().subtract(ray.origin.as_vector())
			.dot(self.normal);
		let denominator = ray.direction.dot(self.normal);

		if denominator == 0.0 {
			// the line and the plane are parallel
			if numerator == 0.0 {
				return Some(0.0);
			} else {
				return None;
			}
		}	else {
			// the line intersects the plane at a single point
			let d = numerator / denominator;

			if d > 0.0 {
				return Some(d);
			} else {
				return None;
			}
		}
	}
}
