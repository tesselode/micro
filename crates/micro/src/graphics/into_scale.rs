use glam::{Vec2, Vec3};

pub trait IntoScale2d {
	fn into_scale_2d(self) -> Vec2;
}

impl IntoScale2d for f32 {
	fn into_scale_2d(self) -> Vec2 {
		Vec2::splat(self)
	}
}

impl IntoScale2d for (f32, f32) {
	fn into_scale_2d(self) -> Vec2 {
		self.into()
	}
}

impl IntoScale2d for [f32; 2] {
	fn into_scale_2d(self) -> Vec2 {
		self.into()
	}
}

impl IntoScale2d for Vec2 {
	fn into_scale_2d(self) -> Vec2 {
		self
	}
}

pub trait IntoScale3d {
	fn into_scale_3d(self) -> Vec3;
}

impl IntoScale3d for f32 {
	fn into_scale_3d(self) -> Vec3 {
		Vec3::splat(self)
	}
}

impl IntoScale3d for (f32, f32, f32) {
	fn into_scale_3d(self) -> Vec3 {
		self.into()
	}
}

impl IntoScale3d for [f32; 3] {
	fn into_scale_3d(self) -> Vec3 {
		self.into()
	}
}

impl IntoScale3d for Vec3 {
	fn into_scale_3d(self) -> Vec3 {
		self
	}
}
