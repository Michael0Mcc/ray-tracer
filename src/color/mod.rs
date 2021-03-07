#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

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

	pub fn af64(&self) -> f64 {
		self.3
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
	
	pub fn a(&self) -> i32 {
		(self.3 *255.999) as i32
	}

	pub fn from_argb(argb: i32) -> Color {
		let a = (argb >> 24) & 0xFF;
		let r = (argb >> 16) & 0xFF;
		let g = (argb >> 8) & 0xFF;
		let b = (argb >> 0) & 0xFF;
		Color((r as f64)/255.0, (g as f64)/255.0, (b as f64)/255.0, (a as f64)/255.0)
	}

	pub fn from_rgb(rgb: i32) -> Color {
		let r = (rgb >> 16) & 0xFF;
		let g = (rgb >> 8) & 0xFF;
		let b = (rgb >> 0) & 0xFF;
		Color((r as f64)/255.0, (g as f64)/255.0, (b as f64)/255.0, 1.0)
	}

	pub fn to_argb(&self) -> i32 {
		(self.a() << 24) + (self.r() << 16) + (self.g() << 8) + (self.b() << 0)
	}

	pub fn to_rgb(&self) -> i32 {
		(self.r() << 16) + (self.g() << 8) + (self.b() << 0)
	}
}

pub fn write_color(pixel_color: Color) {
	print!("{} {} {} \n", pixel_color.r(), pixel_color.g(), pixel_color.b());
}


/********************
*** Color Testing ***
********************/

impl PartialEq for Color {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
	}
}

#[cfg(test)]
mod test;