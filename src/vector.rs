use crate::point::Point3;

#[derive(Copy, Clone)]
pub struct Vector3 {
	x: f32,
	y: f32,
	z: f32,
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 {
			x: x,
			y: y,
			z: z,
		}
	}

	pub fn as_point(&self) -> Point3 {
		Point3::new(self.x, self.y, self.z)
	}

	pub fn equals(&self, v: Vector3) -> bool {
		if self.x == v.x && self.y == v.y && self.z == v.z {
			return true;
		}

		return false;
	}

	pub fn length(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	pub fn scale(&mut self, r: f32) {
		self.x *= r;
		self.y *= r;
		self.z *= r;
	}

	pub fn add(&mut self, v: Vector3) -> &mut Vector3 {
		self.x += v.x;
		self.y += v.y;
		self.z += v.z;

		return self;
	}

	pub fn subtract(&mut self, v: Vector3) -> &mut Vector3 {
		self.x -= v.x;
		self.y -= v.y;
		self.z -= v.z;

		return self;
	}

	pub fn dot(&self, v: Vector3) -> f32 {
		return self.x * v.x + self.y * v.y + self.z * v.z;
	}

	pub fn cross(&self, v: Vector3) -> Vector3 {
		Vector3::new(
			self.y * v.z - self.z * v.y,
			self.z * v.x - self.x * v.z,
			self.x * v.y - self.y * v.x
		)
	}

	pub fn rotate_around_axis(&mut self, axis: Vector3, angle: f32) {
		let len = axis.length();
		let mut unit = axis.clone();

		if len != 1.0 {
			unit.scale(1.0 / len);
		}

		let old_x = self.x;
		let old_y = self.y;
		let old_z = self.z;

		// rotate the vector around the unit vector `axis` as described on
		// en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle

		self.x =
			old_x * (angle.cos() + axis.x * axis.x * (1.0 - angle.cos())) +
			old_y * (axis.x * axis.y * (1.0 - angle.cos()) - axis.z * angle.sin()) +
			old_z * (axis.x * axis.z * (1.0 - angle.cos()) + axis.y * angle.sin());
		self.y =
			old_x * (axis.y * axis.x * (1.0 - angle.cos()) + axis.z * angle.sin()) +
			old_y * (angle.cos() + axis.y * axis.y * (1.0 - angle.cos())) +
			old_z * (axis.y * axis.z * (1.0 - angle.cos()) - axis.x * angle.sin());
		self.z =
			old_x * (axis.z * axis.x * (1.0 - angle.cos()) - axis.y * angle.sin()) +
			old_y * (axis.z * axis.y * (1.0 - angle.cos()) + axis.x * angle.sin()) +
			old_z * (angle.cos() + axis.z * axis.z * (1.0 - angle.cos()));
	}

	pub fn rotate_around_x(&mut self, angle: f32) {
		self.rotate_around_axis(Vector3::new(1.0, 0.0, 0.0), angle);
	}

	pub fn rotate_around_y(&mut self, angle: f32) {
		self.rotate_around_axis(Vector3::new(0.0, 1.0, 0.0), angle);
	}

	pub fn rotate_around_z(&mut self, angle: f32) {
		self.rotate_around_axis(Vector3::new(0.0, 0.0, 1.0), angle);
	}
}
