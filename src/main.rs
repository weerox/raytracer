mod point;
mod vector;
mod ray;
mod color;
mod object;
mod light;
mod camera;
mod scene;

use vector::Vector3;
use point::Point3;
use camera::Camera;
use scene::Scene;
use object::Object;
use object::Sphere;
use object::Plane;
use light::Light;
use light::DirectionalLight;

use std::time::Instant;

fn main() {
	let camera = Camera::new(
		Point3::new(0.0, 0.0, 0.0),
		std::f32::consts::PI / 2.0,
		Vector3::new(1.0, 0.0, 0.0)
	);

	let sphere = Sphere::new(
		Point3::new(4.0, 0.0, -1.0),
		2.0,
		color::RGB_PASTEL_BLUE
	);

	let sphere2 = Sphere::new(
		Point3::new(4.0, 3.0, 0.0),
		1.0,
		color::RGB_PASTEL_RED
	);

	let plane = Plane::new(
		Point3::new(0.0, 0.0, -1.0),
		Vector3::new(0.0, 0.0, 1.0),
		color::RGB_PASTEL_GREEN
	);

	let light = DirectionalLight::new(Vector3::new(0.0, -1.0, -1.0));

	let mut scene = Scene::new(camera);

	scene.add_object(Object::Sphere(sphere));
	scene.add_object(Object::Sphere(sphere2));
	scene.add_object(Object::Plane(plane));

	scene.add_light(Light::Directional(light));

	eprint!("Rendering... ");
	let start = Instant::now();

	let image = scene.render();

	eprintln!("{:?}", start.elapsed());

	eprint!("Saving... ");
	let start = Instant::now();

	image.save("render.png").unwrap();

	eprintln!("{:?}", start.elapsed());
}
