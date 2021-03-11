//Imports

mod utility;
use std::usize;

use utility::*;

pub mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod color;
use color::*;

pub mod hittable;
use hittable::*;
use hittable::hittable_list::*;

pub mod camera;
use camera::*;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
	let mut rec = HitRecord {
		p: Vec3(0.0, 0.0, 0.0),
		normal: Vec3(0.0, 0.0, 0.0),
		t: 0.0,
		front_face: false,
	};

	if world.hit(ray, 0.0, INFINITY, &mut rec) {
		return 0.5 * (Color(1.0, 1.0, 1.0) + rec.normal);
	}

	let unit_direction: Vec3 = ray.direction().normal();
	let t = 0.5*(unit_direction.y() + 1.0);
	(1.0-t)*Color(0.1, 0.1, 0.4) + t*Color(0.0, 0.0, 0.0)
}


// Render

pub fn printimage(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, world: &HittableList) {

	// Image

	// let aspect_ratio: f64 = 4.0 / 3.0;
	// let image_width: i32 = 1200;
	let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;


	// Camera

	let cam = camera(aspect_ratio);


	print!("P3\n{} {}\n255\n", image_width, image_height);

	for j in (0..image_height).rev() {

		// Progress Bar
		let percent = 100.0 - ((j*100) as f64 /image_height as f64);
		eprint!("\r{} - {}%; Scanlines remaining: {}; ", format!("{:*<20}", "█".repeat((percent/5.0) as usize)), percent as i32, j);
		
		for i in 0..image_width {
			let mut pixel_color = Color(0.0, 0.0, 0.0);
			for _s in 0..samples_per_pixel {
				let u = ((i as f64) + random_f64()) / ((image_width - 1) as f64);
				let v = ((j as f64) + random_f64()) / ((image_height - 1) as f64);

				let ray = cam.get_ray(u, v);
				pixel_color += ray_color(&ray, world);
			}
			write_color(pixel_color, samples_per_pixel);
		}
	}

	eprint!("\nDone.\n");
}
