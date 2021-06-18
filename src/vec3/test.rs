use super::*;

#[test]
fn get_x() {
	assert_eq!(4.0, Vec3(4.0, 0.0, 3.2).x());
}

#[test]
fn get_y() {
	assert_eq!(0.0, Vec3(4.0, 0.0, 3.2).y());
}

#[test]
fn get_z() {
	assert_eq!(3.2, Vec3(4.0, 0.0, 3.2).z());
}

#[test]
fn get_len() {
	assert_eq!(13.0, Vec3(12.0, 3.0, 4.0).len());
}

#[test]
fn dot() {
	assert_eq!(60.8, Vec3(4.0, 0.0, 3.2).dot(&Vec3(12.0, 3.0, 4.0)));
}

#[test]
fn cross() {
	assert_eq!(Vec3(-15.0, -2.0, 39.0), Vec3(3.0, -3.0, 1.0).cross(&Vec3(4.0, 9.0, 2.0)));
}

#[test]
fn normal() {
	assert_eq!(Vec3(12.0/13.0, 3.0/13.0, 4.0/13.0), Vec3(12.0, 3.0, 4.0).normal());
}

#[test]
fn neg() {
	assert_eq!(Vec3(-4.0, 0.0, -3.2), -Vec3(4.0, 0.0, 3.2));
}

#[test]
fn add() {
	assert_eq!(Vec3(16.0, 3.0, 7.2), Vec3(4.0, 0.0, 3.2) + Vec3(12.0, 3.0, 4.0));
}

#[test]
fn add_assign() {
	let mut v = Vec3(4.0, 0.0, 3.2);
	v += Vec3(12.0, 3.0, 4.0);
	assert_eq!(Vec3(16.0, 3.0, 7.2), v);
}

#[test]
fn sub() {
	assert_eq!(Vec3(-8.0, -3.0, -1.0), Vec3(4.0, 0.0, 3.0) - Vec3(12.0, 3.0, 4.0));
}

#[test]
fn mul_scalar() {
	assert_eq!(Vec3(12.0, 0.0, 9.0), Vec3(4.0, 0.0, 3.0) * 3.0);
}

#[test]
fn mul_self() {
	assert_eq!(Vec3(48.0, 0.0, 12.0), Vec3(4.0, 0.0, 3.0) * Vec3(12.0, 3.0, 4.0));
}

#[test]
fn mul_scalar_reverse() {
	assert_eq!(Vec3(12.0, 0.0, 9.0), 3.0 * Vec3(4.0, 0.0, 3.0));
}

#[test]
fn mul_assign_scalar() {
	let mut v = Vec3(4.0, 0.0, 3.0);
	v *= 3.0;
	assert_eq!(Vec3(12.0, 0.0, 9.0), v);
}

#[test]
fn mul_assign_self() {
	let mut v = Vec3(4.0, 0.0, 3.0);
	v *= Vec3(12.0, 3.0, 4.0);
	assert_eq!(Vec3(48.0, 0.0, 12.0), v);
}

#[test]
fn div() {
	assert_eq!(Vec3(4.0, 0.0, 3.0), Vec3(12.0, 0.0, 9.0) / 3.0);
}

#[test]
fn div_assign() {
	let mut v = Vec3(12.0, 0.0, 9.0);
	v /= 3.0;
	assert_eq!(Vec3(4.0, 0.0, 3.0), v);
}
