//Imports

mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod color;
use color::*;


// Sets Background Gradient

fn ray_color(ray: Ray) -> Color {
	let unit_direction: Vec3 = ray.direction().normal();
	let gradient = 0.5*(unit_direction.y() + 1.0);
	(1.0-gradient)*Color(1.0, 1.0, 1.0) + gradient*Color(0.5, 0.7, 1.0)
}


// Render

pub fn printimage(aspect_ratio: f64, image_width: i32) {

	// Image

	// let aspect_ratio: f64 = 4.0 / 3.0;
	// let image_width: i32 = 900;
	let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;


	// Camera

	let viewport_height: f64 = 2.0;
	let	viewport_width: f64 = aspect_ratio * viewport_height;
	let focal_length: f64 = 1.0;


	let origin: Point3 = Vec3(0.0, 0.0, 0.0);
	let horizontal: Vec3 = Vec3(viewport_width, 0.0, 0.0);
	let verticle: Vec3 = Vec3(0.0, viewport_height, 0.0);
	let lower_left_corner = origin - horizontal/2.0 - verticle/2.0 - Vec3(0.0, 0.0, focal_length);


	print!("P3\n{} {}\n255\n", image_width, image_height);

	for j in (0..image_height).rev() {
		eprint!("\rScanlines remaining: {}", j);
		for i in 0..image_width {
			let u: f64 = (i as f64) / ((image_width-1) as f64);
			let v = (j as f64) / ((image_height-1) as f64);

			let r: Ray = ray(origin, lower_left_corner + u*horizontal + v*verticle - origin);

			let pixel_color: Color = ray_color(r);
			write_color(pixel_color);
		}
	}

	eprint!("\nDone.\n");
}
