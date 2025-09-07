use glam::Vec2;

use crate::math::Rect;

#[test]
fn positioned() {
	assert_eq!(
		Rect::new((0.0, 0.0), (100.0, 100.0))
			.positioned(Vec2::new(300.0, 300.0), Vec2::new(0.5, 1.0)),
		Rect::new((250.0, 200.0), (100.0, 100.0)),
	)
}

#[test]
fn repositioned() {
	assert_eq!(
		Rect::new((0.0, 0.0), (100.0, 100.0)).resized(Vec2::new(150.0, 200.0), Vec2::new(1.0, 0.5)),
		Rect::new((-50.0, -50.0), (150.0, 200.0)),
	)
}

#[test]
fn padded() {
	assert_eq!(
		Rect::from_corners(Vec2::new(50.0, 50.0), Vec2::new(100.0, 100.0))
			.padded(Vec2::new(10.0, 20.0)),
		Rect::from_corners(Vec2::new(40.0, 30.0), Vec2::new(110.0, 120.0))
	)
}
