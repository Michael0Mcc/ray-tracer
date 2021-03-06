mod vec3;
mod color;
use color::*;

pub fn printimage(imagewidth: usize, imageheight: usize) {

	print!("P3\n{} {}\n255\n", imagewidth, imageheight);

	for j in (0..imageheight).rev() {
		eprintln!("\rScanlines remaining: {}", j);
		for i in 0..imagewidth {
			let pixel_color: Color = Color((i as f64) / ((imagewidth-1) as f64), (j as f64) / ((imageheight-1) as f64), 0.25, 1.0);

			// print!("{} {} {} \n", (r*255.999) as i32, (g*255.999) as i32,(b*255.999) as i32);
			write_color(pixel_color);
		}
	}
}
