//Imports

mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod color;
use color::*;


// P(t) = A+tb
// t^(2)b•b + 2tb•(A−C) + (A−C)•(A−C) − r^(2) = 0
pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
	let oc = ray.origin() - center;

	let a = ray.direction().len_squared(); 
	let half_b = oc.dot(&ray.direction());
	let c = oc.dot(&oc) - radius*radius;

	let discriminant = half_b*half_b - a*c;

	if discriminant < 0.0 {
		return -1.0;
	}
	(-half_b - discriminant.sqrt()) / a
}

fn ray_color(ray: &Ray) -> Color {
	// Sphere Hit
	let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, ray);
	if t > 0.0 {
		let normal: Vec3 = (ray.at(t) - Vec3(0.0, 0.0, -1.0)).normal();
		return 0.5*Color(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0);
	}

	let unit_direction: Vec3 = ray.direction().normal();
	let gradient = 0.5*(unit_direction.y() + 1.0);
	(1.0-gradient)*Color(0.1, 0.1, 0.4) + gradient*Color(0.0, 0.0, 0.0)
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

			let pixel_color: Color = ray_color(&r);
			write_color(pixel_color);
		}
	}

	eprint!("\nDone.\n");
}
