use super::*;

pub struct HittableList {
	pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add<T: Hittable + 'static>(&mut self, object: T) {
		self.objects.push(Box::new(object));
	}
}

impl Hittable for HittableList {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut temp_rec: HitRecord = HitRecord{
			p: Vec3(0.0, 0.0, 0.0),
			normal: Vec3(0.0, 0.0, 0.0),
			t: 0.0,
			front_face: false,
		};

		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				rec.p = temp_rec.p;
				rec.normal = temp_rec.normal;
				rec.t = temp_rec.t;
				rec.front_face = temp_rec.front_face;
			}
		}

		hit_anything
	}
}