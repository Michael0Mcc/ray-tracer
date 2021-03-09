#![allow(dead_code)]

use super::vec3::*;
use super::ray::*;

pub struct Camera {
	aspect_ratio: f64,
	viewport_height: f64,
	viewport_width: f64,
	focal_length: f64,

	origin: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	lower_left_corner: Vec3,
}

pub fn camera(aspect_ratio: f64) -> Camera {
	let viewport_height = 2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;

	let origin = Vec3::empty();
	let horizontal = Vec3(viewport_width, 0.0, 0.0);
	let vertical = Vec3(0.0, viewport_height, 0.0);
	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

	Camera {
		aspect_ratio,
		viewport_height,
		viewport_width,
		focal_length,

		origin,
		horizontal,
		vertical,
		lower_left_corner,
	}
}

impl Camera {
	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray {
			origin: self.origin,
			direction: self.lower_left_corner + u*self.horizontal + v*self.vertical + self.origin,
		}	
	}
}