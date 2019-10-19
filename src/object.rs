use crate::point::Point3;
use crate::vector::Vector3;
use crate::ray::Ray;
use crate::color::Rgb;
use crate::color::RGB_BLACK;
use crate::scene::Scene;
use crate::light::Light;

#[derive(Copy, Clone)]
pub enum Object {
	Sphere(Sphere),
	Plane(Plane),
}

impl Object {
	pub fn intersects(&self, ray: &Ray) -> Option<f32> {
		match self {
			Object::Sphere(object) => object.intersects(ray),
			Object::Plane(object) => object.intersects(ray),
		}
	}

	pub fn color(&self, scene: &Scene, point: Point3) -> Rgb {
		let color = match self {
			Object::Sphere(object) => object.color,
			Object::Plane(object) => object.color,
		};

		for i in 0..scene.lights.len() {
			let light = scene.lights[i];
			let mut direction = match light {
				Light::Directional(light) => {
					let mut d = light.direction;
					d.scale(-1.0);
					d
				},
				Light::Point(light) => {
					light.point.as_vector().subtract(point.as_vector()).clone()
				},
			};

			if direction.length() != 1.0 {
				direction.scale(1.0 / direction.length());
			}

			let ray = Ray::new(point, direction);
			let mut intersects = false;

			for j in 0..scene.objects.len() {
				let object = scene.objects[j];

				if object.intersects(&ray).is_some() {
					intersects = true;
					break;
				}
			}

			if intersects {
				return RGB_BLACK;
			}
		}

		return color;
	}

	pub fn normal(&self, point: Point3) -> Vector3 {
		match self {
			Object::Sphere(object) => object.normal(point),
			Object::Plane(object) => object.normal(point),
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

	pub fn intersects(&self, ray: &Ray) -> Option<f32> {
		let mut ray = ray.clone();
		// make sure that the ray is a unit vector
		if ray.direction.length() != 1.0 {
			ray.direction.scale(1.0 / ray.direction.length());
		}

		let mut center_to_origin = ray.origin.as_vector();
		center_to_origin.subtract(self.center.as_vector());

		// formula from en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

		let a = ray.direction.dot(ray.direction);
		let b = 2.0 * ray.direction.dot(center_to_origin);
		let c = center_to_origin.dot(center_to_origin) - self.radius * self.radius;

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
			let d1 = (-b + discriminant.sqrt()) / (2.0 * a);
			let d2 = (-b - discriminant.sqrt()) / (2.0 * a);

			if d1 > 0.0001 && d2 > 0.0001 {
				return Some(d1.min(d2));
			} else if d1 <= 0.0001 && d2 <= 0.0001 {
				return None;
			} else {
				return Some(d1.max(d2));
			}
		}

		return None;
	}

	pub fn normal(&self, point: Point3) -> Vector3 {
		let mut p = point.as_vector();
		let c = self.center.as_vector();
		let mut normal = p.subtract(c).clone();

		if normal.length() != 1.0 {
			normal.scale(1.0 / normal.length());
		}

		return normal;
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

	pub fn intersects(&self, ray: &Ray) -> Option<f32> {
		let mut ray = ray.clone();
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

			if d > 0.0001 {
				return Some(d);
			} else {
				return None;
			}
		}
	}

	pub fn normal(&self, point: Point3) -> Vector3 {
		let mut normal = self.normal.clone();

		if normal.length() != 1.0 {
			normal.scale(1.0 / normal.length());
		}

		return normal;
	}
}
