use std::ops::RangeInclusive;

use glam::{Mat4, Vec3};

use crate::{Context, Push, context::OnDrop, math::Rect};

/// Settings for a 3D camera.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3d {
	/// The kind of projection the camera uses.
	pub kind: Camera3dKind,
	/// The minimum draw distance to use.
	pub z_near: f32,
	/// The maximum draw distance to use.
	pub z_far: f32,
	/// The location of the camera.
	pub position: Vec3,
	/// Where the camera is pointed.
	pub look_at: Vec3,
}

impl Camera3d {
	/**
	Creates a new camera with a perspective projection.

	This is the standard projection where more distant objects appear
	smaller, just like when you look at things with your eyes
	in real life.

	- `field_of_view` determines the viewing angle of the camera.
	- `aspect_ratio` should be width / height for the surface you
	  plan to draw on with this camera
	- `z_bounds` is the minimum and maximuum draw distance.
	- `position` is the position of the camera.
	- `look_at` is where the camera is pointed.
	*/
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

	/**
	Creates a new camera with a perspective projection where "up" is in
	the negative Y direction.

	This is the standard projection where more distant objects appear
	smaller, just like when you look at things with your eyes
	in real life.

	- `field_of_view` determines the viewing angle of the camera.
	- `aspect_ratio` should be width / height for the surface you
	  plan to draw on with this camera
	- `z_bounds` is the minimum and maximuum draw distance.
	- `position` is the position of the camera.
	- `look_at` is where the camera is pointed.
	*/
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

	/**
	Creates a new camera with an ortographic projection.

	In this projection, the size of an object is not affected by its
	distance from the camera.

	- `xy_bounds` is the rectangular region that should be visible in the
	  camera.
	- `z_bounds` is the minimum and maximuum draw distance.
	- `position` is the position of the camera.
	- `look_at` is where the camera is pointed.
	*/
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

	/// Returns the direction of "up" as a number (either `1.0` or `-1.0`).
	pub fn up_y(self) -> f32 {
		match self.kind {
			Camera3dKind::Perspective { up_y, .. } => up_y,
			Camera3dKind::Orthographic { .. } => 1.0,
		}
	}

	/// Returns the projection matrix.
	pub fn projection(self) -> Mat4 {
		match self.kind {
			Camera3dKind::Perspective {
				field_of_view,
				aspect_ratio,
				..
			} => Mat4::perspective_rh(field_of_view, aspect_ratio, self.z_near, self.z_far),
			Camera3dKind::Orthographic { xy_bounds } => Mat4::orthographic_rh(
				xy_bounds.left(),
				xy_bounds.right(),
				xy_bounds.bottom(),
				xy_bounds.top(),
				self.z_near,
				self.z_far,
			),
		}
	}

	/// Returns the view matrix.
	pub fn view(self) -> Mat4 {
		let up_y = self.up_y();
		Mat4::look_at_rh(self.position, self.look_at, Vec3::new(0.0, up_y, 0.0))
	}

	/// Returns a transformation that can be passed to [`Context::push`] to use
	/// this camera for drawing operations.
	pub fn transform(self, ctx: &Context) -> Mat4 {
		let _span = tracy_client::span!();
		Self::undo_2d_coordinate_system_transform(ctx) * self.projection() * self.view()
	}

	/// Pushes this camera's transformation to the graphics stack.
	pub fn push(self, ctx: &'_ mut Context) -> OnDrop<'_> {
		ctx.push(Push {
			transform: Some(self.transform(ctx)),
			enable_depth_testing: Some(true),
			..Default::default()
		})
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

/// The types of projection a [`Camera3d`] can have.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Camera3dKind {
	/**
	This is the standard projection where more distant objects appear
	smaller, just like when you look at things with your eyes
	in real life.
	*/
	Perspective {
		/// Determines the viewing angle of the camera.
		field_of_view: f32,
		/// Should be width / height for the surface you plan to draw on
		/// with this camera.
		aspect_ratio: f32,
		/// `1.0` if positive Y is up, or `-1.0` is negative Y is up.
		up_y: f32,
	},
	/// In this projection, the size of an object is not affected by its
	/// distance from the camera.
	Orthographic {
		/// The rectangular region that should be visible in the camera.
		xy_bounds: Rect,
	},
}
