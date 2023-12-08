use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};
use palette::LinSrgba;

use super::color_constants::ColorConstants;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceParams {
	pub transform: Mat4,
	pub normal_transform: Mat4,
	pub color: LinSrgba,
}

impl InstanceParams {
	pub fn new() -> Self {
		Self {
			transform: Mat4::IDENTITY,
			normal_transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	pub fn transformed(self, transform: Mat4) -> Self {
		let new_transform = transform * self.transform;
		Self {
			transform: new_transform,
			normal_transform: new_transform.inverse().transpose(),
			..self
		}
	}

	pub fn translated_2d(self, translation: Vec2) -> Self {
		self.transformed(Mat4::from_translation(translation.extend(0.0)))
	}

	pub fn translated_3d(self, translation: Vec3) -> Self {
		self.transformed(Mat4::from_translation(translation))
	}

	pub fn scaled_2d(self, scale: Vec2) -> Self {
		self.transformed(Mat4::from_scale(scale.extend(1.0)))
	}

	pub fn scaled_3d(self, scale: Vec3) -> Self {
		self.transformed(Mat4::from_scale(scale))
	}

	pub fn rotated_x(self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_x(rotation))
	}

	pub fn rotated_y(self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_y(rotation))
	}

	pub fn rotated_z(self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_z(rotation))
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}
}

impl Default for InstanceParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Vec2> for InstanceParams {
	fn from(position: Vec2) -> Self {
		Self::new().translated_2d(position)
	}
}

impl From<Vec3> for InstanceParams {
	fn from(position: Vec3) -> Self {
		Self::new().translated_3d(position)
	}
}

impl From<Mat4> for InstanceParams {
	fn from(transform: Mat4) -> Self {
		Self::new().transformed(transform)
	}
}

impl From<LinSrgba> for InstanceParams {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}
