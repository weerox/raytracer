use crate::camera::Camera;
use crate::object::Object;
use crate::ray::Ray;
use crate::color::Rgb;

use image::{ImageBuffer, RgbImage};

pub struct Scene {
	camera: Camera,
	objects: Vec<Object>,
}

impl Scene {
	pub fn new(camera: Camera) -> Scene {
		Scene {
			camera: camera,
			objects: Vec::new(),
		}
	}

	pub fn add_object(&mut self, object: Object) {
		self.objects.push(object);
	}

	pub fn render(&self) -> RgbImage {
		let width = 800;
		let height = 800;
		let mut image: RgbImage = ImageBuffer::new(width, height);

		let mut up_start = self.camera.up.clone();
		let mut left_start = self.camera.up.cross(self.camera.direction);

		let left_len = (self.camera.field_of_view / 2.0).tan();
		left_start.scale(left_len / left_start.length());

		let up_len = (self.camera.field_of_view / 2.0).tan();
		up_start.scale(up_len / up_start.length());

		let delta_x = (left_len * 2.0) / width as f32;
		let delta_y = (up_len * 2.0) / height as f32;

		for x in 0..width {
			for y in 0..height {
				let mut ray_direction = self.camera.direction.clone();

				let mut left = left_start.clone();
				left.scale((left_len - (x as f32 + 0.5) * delta_x) / left.length());

				let mut up = up_start.clone();
				up.scale((up_len - (y as f32 + 0.5) * delta_y) / up.length());

				ray_direction.add(left);
				ray_direction.add(up);

				let ray = Ray::new(self.camera.position, ray_direction);
				let mut min_d = std::f32::MAX;
				let mut min_object = None;
				for i in 0..self.objects.len() {
					let object = self.objects[i];
					let d = object.intersects(ray);

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

					image.put_pixel(x, y, min_object.color().to_pixel());
				} else {
					image.put_pixel(x, y, Rgb::new(0, 0, 0).to_pixel());
				}
			}
		}

		return image;
	}
}
