use crate::point::Point3;
use crate::vector::Vector3;
use crate::scene::Scene;
use crate::color::Rgb;
use crate::color::RGB_BLACK;

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

	pub fn trace(&self, scene: &Scene) -> Rgb {
		let mut min_d = std::f32::MAX;
		let mut min_object = None;
		for i in 0..scene.objects.len() {
			let object = scene.objects[i];
			let d = object.intersects(self);

			if d != None {
				let d = d.unwrap();
				if d < min_d {
					min_d = d;
					min_object = Some(object);
				}
			}
		}

		if min_object.is_some() {
			let min_object = min_object.unwrap();

			let mut point = self.direction.clone();
			point.scale(min_d / point.length());

			// move the point a tiny distance along the normal
			// to make sure that it doesn't end up inside/behind objects
			// due to floating point errors
			let mut normal = min_object.normal(point.as_point());
			normal.scale(0.000001 / normal.length());

			point.add(normal);

			return min_object.color(scene, point.as_point());
		} else {
			return RGB_BLACK;
		}
	}
}
