use super::*;

#[test]
fn to_from_argb() {
	assert_eq!(0x0F963AFF, Color::from_argb(0x0F963AFF).to_argb());	
}

#[test]
fn to_from_rgb() {
	assert_eq!(0x963AFF, Color::from_argb(0x963AFF).to_argb());	
}