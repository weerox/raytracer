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

	pub fn length(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	pub fn scale(&mut self, r: f32) {
		self.x *= r;
		self.y *= r;
		self.z *= r;
	}

	pub fn add(&mut self, v: Vector3) {
		self.x += v.x;
		self.y += v.y;
		self.z += v.z;
	}

	pub fn subtract(&mut self, v: Vector3) {
		self.x -= v.x;
		self.y -= v.y;
		self.z -= v.z;
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
}

#[derive(Clone)]
pub struct Vector2 {
	x: f32,
	y: f32,
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Vector2 {
		Vector2 {
			x: x,
			y: y,
		}
	}

	pub fn length(&self) -> f32 {
		return (self.x * self.x + self.y * self.y).sqrt();
	}

	pub fn scale(&mut self, r: f32) {
		self.x *= r;
		self.y *= r;
	}

	pub fn add(&mut self, v: Vector2) {
		self.x += v.x;
		self.y += v.y;
	}

	pub fn subtract(&mut self, v: Vector2) {
		self.x -= v.x;
		self.y -= v.y;
	}

	pub fn dot(&self, v: Vector2) -> f32 {
		return self.x * v.x + self.y * v.y;
	}
}
