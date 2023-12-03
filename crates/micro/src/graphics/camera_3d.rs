use std::f32::consts::FRAC_PI_4;

use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3d {
	pub field_of_view: f32,
	pub aspect_ratio: f32,
	pub z_near: f32,
	pub z_far: f32,
	pub position: Vec3,
	pub look_at: Vec3,
}

impl Camera3d {
	pub fn transform(self) -> Mat4 {
		Mat4::perspective_rh(
			self.field_of_view,
			self.aspect_ratio,
			self.z_near,
			self.z_far,
		) * Mat4::look_at_rh(self.position, self.look_at, Vec3::new(0.0, 1.0, 0.0))
	}
}

impl Default for Camera3d {
	fn default() -> Self {
		Self {
			field_of_view: FRAC_PI_4,
			aspect_ratio: 4.0 / 3.0,
			z_near: 0.0,
			z_far: 100.0,
			position: Vec3::ZERO,
			look_at: Vec3::new(0.0, 0.0, 1.0),
		}
	}
}
