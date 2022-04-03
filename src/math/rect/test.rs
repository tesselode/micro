use vek::Vec2;

use crate::math::Rect;

#[test]
fn padded() {
	assert_eq!(
		Rect::new(Vec2::new(50.0, 50.0), Vec2::new(100.0, 100.0)).padded(Vec2::new(10.0, 20.0)),
		Rect::new(Vec2::new(40.0, 30.0), Vec2::new(110.0, 120.0))
	)
}
