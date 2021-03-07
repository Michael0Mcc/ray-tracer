use super::*;

#[test]
fn test_at() {
	assert_eq!(Vec3(12.0, 0.0, 9.0), ray(Vec3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 3.0)).at(3.0));
}

