#![allow(dead_code)]

use rand::{self, Rng};


// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;


// Functions

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
	if x < min { return min };
	if x > max { return max };

	x
}

pub fn random_f64()-> f64 {
	let mut rng = rand::thread_rng();
	let o: f64 = rng.gen();

	o
}

pub fn random_range_f64(min: f64, max: f64) -> f64 {
	min + (max-min)*random_f64()
}