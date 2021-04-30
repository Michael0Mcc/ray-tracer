#![allow(dead_code)]

use super::color::Color;
use super::utility::{ random_f64, random_range_f64 };

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);


pub fn rand_in_unit_sphere() -> Vec3 {
	loop {
		let p: Vec3 = Vec3::vec3_rand_range(-1.0, 1.0);
		if p.len_squared() >= 1.0 { continue; }
		return p;
	}
}


impl Vec3 {
	pub fn empty() -> Vec3 {
		Vec3(0.0, 0.0, 0.0)
	}

	pub fn from_scalar(t: f64) -> Vec3 {
		Vec3(t, t, t)
	}

	pub fn clone(&self) -> Vec3 {
		Vec3(self.0, self.1, self.2)
	}

	pub fn vec3_rand() -> Vec3 {
		Vec3(random_f64(), random_f64(), random_f64())
	}

	pub fn vec3_rand_range(min: f64, max: f64) -> Vec3 {
		Vec3(random_range_f64(min, max), random_range_f64(min, max), random_range_f64(min, max))
	}

	pub fn x(&self) -> f64 { self.0 }
	pub fn y(&self) -> f64 { self.1 }
	pub fn z(&self) -> f64 { self.2 }

	pub fn len(&self) -> f64 {
		self.len_squared().sqrt()
	}

	pub fn len_squared(&self) -> f64 {
		self.0*self.0 + self.1*self.1 +self.2*self.2
	}

	pub fn dot(&self, other: &Self) -> f64 {
		self.0*other.0 + self.1*other.1 + self.2*other.2
	}

	pub fn cross(&self, other: &Self) -> Vec3 {
		Vec3(self.1*other.2 - self.2*other.1,
			 self.2*other.0 - self.0*other.2,
			 self.0*other.1 - self.1*other.0
		)
	}

	pub fn normal(&self) -> Vec3 {
		self.clone() / self.len()
	}
}


/********************************
*** Vec3 Operator Overloading ***
********************************/

use std::ops::*;


impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec3(-self.0, -self.1, -self.2)
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self= Vec3(self.0+other.0, self.1+other.1, self.2+other.2);
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vec3(self.0+other.0, self.1+other.1, self.2+other.2)
	}
}

impl Add<Color> for Vec3 {
	type Output = Self;

	fn add(self, other: Color) -> Self {
		Vec3(self.0+other.0, self.1+other.1, self.2+other.2)
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Vec3(self.0-other.0, self.1-other.1, self.2-other.2)
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, t: f64) {
		*self = Vec3(self.0*t, self.1*t, self.2*t);
	}
}

impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, other: Self) {
		*self = Vec3(self.0*other.0, self.1*other.1, self.2*other.2);
	}
}

impl Mul<f64> for Vec3 {
	type Output = Self;

	fn mul(self, t: f64) -> Self {
		Vec3(self.0*t, self.1*t, self.2*t)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;
	fn mul(self, v: Vec3) -> Vec3 {
		Vec3(v.0*self, v.1*self, v.2*self)
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Vec3(self.0*other.0, self.1*other.1, self.2*other.2)
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, t: f64) {
		*self = Vec3(self.0/t, self.1/t, self.2/t);
	}
}

impl Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, t: f64) -> Self {
		Vec3(self.0/t, self.1/t, self.2/t)
	}
}


/*******************
*** Vec3 Aliases ***
*******************/

pub type Point3 = Vec3;



/*******************
*** Vec3 Testing ***
*******************/

impl PartialEq for Vec3 {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1 && self.2 == other.2
	}
}

#[cfg(test)]
mod test;

