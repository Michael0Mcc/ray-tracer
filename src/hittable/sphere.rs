use core::f64;

use super::*;

pub struct Sphere {
	center: Point3,
	radius: f64
}

pub fn sphere(center: Point3, radius: f64) -> Sphere {
	Sphere { center, radius }
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let oc = ray.origin() - self.center;

		let a = ray.direction().len_squared(); 
		let half_b = oc.dot(&ray.direction());
		let c = oc.dot(&oc) - self.radius*self.radius;

		let discriminant = half_b*half_b - a*c;

		if discriminant < 0.0 { return false; }

		let sqrtd = discriminant.sqrt();

		// Find the nearest root that lies in the acceptable range.
		let mut root = (-half_b - sqrtd) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root {
				return false;
			}
		}

		rec.t = root;
		rec.p = ray.at(rec.t);
		let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
		rec.set_face_normal(ray, &outward_normal);

		true
	}
}