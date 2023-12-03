use std::ops::RangeInclusive;

use glam::{Mat4, Vec3};

use crate::math::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3d {
	pub kind: Camera3dKind,
	pub z_near: f32,
	pub z_far: f32,
	pub position: Vec3,
	pub look_at: Vec3,
}

impl Camera3d {
	pub fn perspective(
		field_of_view: f32,
		aspect_ratio: f32,
		z_bounds: RangeInclusive<f32>,
		position: Vec3,
		look_at: Vec3,
	) -> Self {
		Self {
			kind: Camera3dKind::Perspective {
				field_of_view,
				aspect_ratio,
			},
			z_near: *z_bounds.start(),
			z_far: *z_bounds.end(),
			position,
			look_at,
		}
	}

	pub fn orthographic(
		xy_bounds: Rect,
		z_bounds: RangeInclusive<f32>,
		position: Vec3,
		look_at: Vec3,
	) -> Self {
		Self {
			kind: Camera3dKind::Orthographic { xy_bounds },
			z_near: *z_bounds.start(),
			z_far: *z_bounds.end(),
			position,
			look_at,
		}
	}

	pub fn transform(self) -> Mat4 {
		let projection = match self.kind {
			Camera3dKind::Perspective {
				field_of_view,
				aspect_ratio,
			} => Mat4::perspective_rh(field_of_view, aspect_ratio, self.z_near, self.z_far),
			Camera3dKind::Orthographic { xy_bounds } => Mat4::orthographic_rh(
				xy_bounds.left(),
				xy_bounds.right(),
				xy_bounds.bottom(),
				xy_bounds.top(),
				self.z_near,
				self.z_far,
			),
		};
		projection * Mat4::look_at_rh(self.position, self.look_at, Vec3::new(0.0, 1.0, 0.0))
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Camera3dKind {
	Perspective {
		field_of_view: f32,
		aspect_ratio: f32,
	},
	Orthographic {
		xy_bounds: Rect,
	},
}
