use ray_tracer::printimage;

use ray_tracer::hittable::{ hittable_list::*, sphere::* };
use ray_tracer::vec3::*;


// Image

const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH: i32 = 900;
const SAMPLES_PER_PIXEL: i32 = 100;


fn main() {

	// World
	let mut world = HittableList{ objects: vec![] };
	world.add(sphere(Vec3(0.0, 0.0, -1.0), 0.5));
	world.add(sphere(Vec3(0.0, -100.5, -1.0), 100.0));

	
	printimage(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, &world);
}
