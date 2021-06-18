use super::*;

#[test]
fn to_from_argb() {
	assert_eq!(0x963AFF, Color::from_rgb(0x963AFF).to_rgb());	
}