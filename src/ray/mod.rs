#![allow(dead_code)]

use super::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub origin: Point3,
	pub direction: Vec3
}

pub fn ray(origin: Point3, direction: Vec3) -> Ray {
	Ray { origin, direction }
}

impl Ray {
	pub fn origin(&self) -> Point3 {
		self.origin
	}

	pub fn direction(&self) -> Vec3 {
		self.direction
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.origin() + t*self.direction()
	}
}

#[cfg(test)]
mod test;