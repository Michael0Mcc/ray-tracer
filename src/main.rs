use ray_tracer::printimage;

// Image

const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH: i32 = 900;


fn main() {
	printimage(ASPECT_RATIO, IMAGE_WIDTH)
}
