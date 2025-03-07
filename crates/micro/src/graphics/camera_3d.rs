use std::ops::RangeInclusive;

use glam::{Mat4, Vec3};

use crate::{Context, math::Rect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3d {
	pub kind: Camera3dKind,
	pub z_near: f32,
	pub z_far: f32,
	pub position: Vec3,
	pub look_at: Vec3,
}

impl Camera3d {
	pub const fn perspective(
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
				up_y: 1.0,
			},
			z_near: *z_bounds.start(),
			z_far: *z_bounds.end(),
			position,
			look_at,
		}
	}

	pub const fn perspective_up_y_negative(
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
				up_y: -1.0,
			},
			z_near: *z_bounds.start(),
			z_far: *z_bounds.end(),
			position,
			look_at,
		}
	}

	pub const fn orthographic(
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

	pub fn transform(self, ctx: &Context) -> Mat4 {
		let _span = tracy_client::span!();
		let (projection, up_y) = match self.kind {
			Camera3dKind::Perspective {
				field_of_view,
				aspect_ratio,
				up_y,
			} => (
				Mat4::perspective_rh(field_of_view, aspect_ratio, self.z_near, self.z_far),
				up_y,
			),
			Camera3dKind::Orthographic { xy_bounds } => (
				Mat4::orthographic_rh(
					xy_bounds.left(),
					xy_bounds.right(),
					xy_bounds.bottom(),
					xy_bounds.top(),
					self.z_near,
					self.z_far,
				),
				1.0,
			),
		};
		let view = Mat4::look_at_rh(self.position, self.look_at, Vec3::new(0.0, up_y, 0.0));
		Self::undo_2d_coordinate_system_transform(ctx) * projection * view
	}

	fn undo_2d_coordinate_system_transform(ctx: &Context) -> Mat4 {
		let current_render_target_size = ctx.current_render_target_size();
		(Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
			* Mat4::from_scale(Vec3::new(
				2.0 / current_render_target_size.x as f32,
				-2.0 / current_render_target_size.y as f32,
				1.0,
			)))
		.inverse()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Camera3dKind {
	Perspective {
		field_of_view: f32,
		aspect_ratio: f32,
		up_y: f32,
	},
	Orthographic {
		xy_bounds: Rect,
	},
}
