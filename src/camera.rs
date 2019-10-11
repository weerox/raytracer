use crate::point::Point3;
use crate::vector::Vector3;

pub struct Camera {
	pub position: Point3,
	pub field_of_view: f32,
	pub direction: Vector3,
	pub up: Vector3,
}

impl Camera {
	pub fn new(
		position: Point3,
		field_of_view: f32,
		direction: Vector3
	) -> Camera {
		let mut direction = direction;
		let len = direction.length();

		// the `base_direction` vector can be seen as the camera direction rotated
		// to line up with the x-axis

		let base_up = Vector3::new(0.0, 0.0, 1.0);

		let mut base_direction = Vector3::new(1.0, 0.0, 0.0);
		base_direction.scale(len);

		// we can now figure out how to rotate the `base_direction` vector
		// so that it becomes the `direction`

		// we can then figure out the new `up` direction by taking the sum of 
		// the `base_direction` and the `up` direction and apply
		// the same rotation

		// the new `up` direction is the difference between `direction` and `sum`

		let mut sum = base_direction.clone();
		sum.add(base_up);

		let normal = base_direction.cross(direction);

		let angle =
			(normal.length() / (base_direction.length() * direction.length())).asin();

		sum.rotate_around_axis(normal, angle);

		let mut up = sum.clone();
		up.subtract(direction);

		// convert to unit vectors
		up.scale(1.0 / up.length());
		direction.scale(1.0 / direction.length());

		Camera {
			position: position,
			field_of_view: field_of_view,
			direction: direction,
			up: up,
		}
	}
}
