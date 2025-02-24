use glam::{IVec2, Vec2};

pub trait VecConstants {
	const UP: Self;
	const DOWN: Self;
	const LEFT: Self;
	const RIGHT: Self;
}

impl VecConstants for Vec2 {
	const UP: Self = Vec2::new(0.0, -1.0);
	const DOWN: Self = Vec2::new(0.0, 1.0);
	const LEFT: Self = Vec2::new(-1.0, 0.0);
	const RIGHT: Self = Vec2::new(1.0, 0.0);
}

impl VecConstants for IVec2 {
	const UP: Self = IVec2::new(0, -1);
	const DOWN: Self = IVec2::new(0, 1);
	const LEFT: Self = IVec2::new(-1, 0);
	const RIGHT: Self = IVec2::new(1, 0);
}
