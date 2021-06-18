#![allow(dead_code)]

use super::{ ray::*, vec3::* };

pub mod hittable_list;
pub mod sphere;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
		self.front_face = ray.direction().dot(&outward_normal) < 0.0;
		self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
	}

	pub fn copy(&self) -> HitRecord {
		HitRecord {
			p: self.p,
			normal: self.normal,
			t: self.t,
			front_face: self.front_face
		}
	}
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}