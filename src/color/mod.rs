#![allow(dead_code)]

use std::ops::{ Mul, Add, AddAssign };
use super::vec3::Vec3;
use super::utility::clamp;

#[derive(Debug, Copy, Clone)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
	pub fn rf64(&self) -> f64 {
		self.0
	}

	pub fn gf64(&self) -> f64 {
		self.1
	}

	pub fn bf64(&self) -> f64 {
		self.2
	}

	pub fn r(&self) -> i32 {
		(self.0 *255.999) as i32
	}

	pub fn g(&self) -> i32 {
		(self.1 *255.999) as i32
	}
	
	pub fn b(&self) -> i32 {
		(self.2 *255.999) as i32
	}

	pub fn from_rgb(rgb: i32) -> Color {
		let r = (rgb >> 16) & 0xFF;
		let g = (rgb >> 8) & 0xFF;
		let b = (rgb >> 0) & 0xFF;
		Color((r as f64)/255.0, (g as f64)/255.0, (b as f64)/255.0)
	}

	pub fn to_rgb(&self) -> i32 {
		(self.r() << 16) + (self.g() << 8) + (self.b() << 0)
	}
}

impl Mul<f64> for Color {
	type Output = Self;

	fn mul(self, t: f64) -> Self {
		Color(self.0*t, self.1*t, self.2*t)
	}
}

impl Mul<Color> for f64 {
	type Output = Color;
	fn mul(self, v: Color) -> Color {
		Color(v.0*self, v.1*self, v.2*self)
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Self) {
		*self= Color(self.0+other.0, self.1+other.1, self.2+other.2);
	}
}

impl Add<Color> for Color {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Color(self.0+other.0, self.1+other.1, self.2+other.2)
	}
}

impl Add<Vec3> for Color {
	type Output = Self;

	fn add(self, other: Vec3) -> Self {
		Color(self.0+other.0, self.1+other.1, self.2+other.2)
	}
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
	let mut r = pixel_color.0;
	let mut g = pixel_color.1;
	let mut b = pixel_color.2;

	let scale = 1.0 / (samples_per_pixel as f64);
	r *= scale;
	g *= scale;
	b *= scale;

	print!("{} {} {} \n",
		(256.0*clamp(r, 0.0, 0.999)) as i32,
		(256.0*clamp(g, 0.0, 0.999)) as i32,
		(256.0*clamp(b, 0.0, 0.999)) as i32
	);
}


/********************
*** Color Testing ***
********************/

impl PartialEq for Color {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1 && self.2 == other.2
	}
}

#[cfg(test)]
mod test;