use crate::camera::Camera;
use crate::object::Object;

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
}
