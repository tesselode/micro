//! Types related to drawing graphics.

mod blend_mode;
mod camera_3d;
pub mod canvas;
mod into_index_range;
mod into_instance_range;
mod into_scale;
pub mod mesh;
mod shader;
pub mod sprite_batch;
mod stencil;
mod storage_buffer;
pub mod text;
pub mod texture;
mod vertex;

pub use blend_mode::*;
pub use camera_3d::*;
pub use canvas::{Canvas, CanvasSettings, RenderToCanvasSettings};
pub use into_index_range::*;
pub use into_instance_range::*;
pub use into_scale::*;
pub use shader::*;
pub use stencil::*;
pub use storage_buffer::*;
pub use vertex::*;
pub use wgpu::{DepthBiasState, Features, PresentMode, TextureFormat};

/// Adds common methods to a drawable object for changing how the object is drawn.
#[macro_export]
macro_rules! standard_draw_param_methods {
	() => {
		/// Applies the specified `transform` to the object when drawing.
		pub fn transformed(&self, transform: impl Into<$crate::math::Mat4>) -> Self {
			let mut new = self.clone();
			new.transform = transform.into() * self.transform;
			new
		}

		/// Moves the object along the X and Y axes by the specified amount when drawing.
		pub fn translated_2d(&self, translation: impl Into<$crate::math::Vec2>) -> Self {
			self.transformed(Mat4::from_translation(translation.into().extend(0.0)))
		}

		/// Moves the object along the X, Y, and Z axes by the specified amount when drawing.
		pub fn translated_3d(&self, translation: impl Into<$crate::math::Vec3>) -> Self {
			self.transformed(Mat4::from_translation(translation.into()))
		}

		/// Moves the object along the X axis by the specified amount when drawing.
		pub fn translated_x(&self, translation: f32) -> Self {
			self.translated_2d($crate::math::Vec2::new(translation, 0.0))
		}

		/// Moves the object along the Y axis by the specified amount when drawing.
		pub fn translated_y(&self, translation: f32) -> Self {
			self.translated_2d($crate::math::Vec2::new(0.0, translation))
		}

		/// Moves the object along the Z axis by the specified amount when drawing.
		pub fn translated_z(&self, translation: f32) -> Self {
			self.translated_3d($crate::math::Vec3::new(0.0, 0.0, translation))
		}

		/// Scales the object along the X and Y axes by the specified amount when drawing.
		pub fn scaled_2d(&self, scale: impl $crate::graphics::IntoScale2d) -> Self {
			self.transformed(Mat4::from_scale(scale.into_scale_2d().extend(1.0)))
		}

		/// Scales the object along the X, Y, and Z axes by the specified amount when drawing.
		pub fn scaled_3d(&self, scale: impl $crate::graphics::IntoScale3d) -> Self {
			self.transformed(Mat4::from_scale(scale.into_scale_3d()))
		}

		/// Scales the object along the X axis by the specified amount when drawing.
		pub fn scaled_x(&self, scale: f32) -> Self {
			self.scaled_2d($crate::math::vec2(scale, 1.0))
		}

		/// Scales the object along the Y axis by the specified amount when drawing.
		pub fn scaled_y(&self, scale: f32) -> Self {
			self.scaled_2d($crate::math::vec2(1.0, scale))
		}

		/// Scales the object along the Z axis by the specified amount when drawing.
		pub fn scaled_z(&self, scale: f32) -> Self {
			self.scaled_3d($crate::math::vec3(1.0, 1.0, scale))
		}

		/// Rotates the object by the specified rotation when drawing.
		pub fn rotated(&self, rotation: $crate::math::Quat) -> Self {
			self.transformed(Mat4::from_quat(rotation))
		}

		/// Rotates the object around the X axis by the specified amount (in radians)
		/// when drawing.
		pub fn rotated_x(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_x(rotation))
		}

		/// Rotates the object around the Y axis by the specified amount (in radians)
		/// when drawing.
		pub fn rotated_y(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_y(rotation))
		}

		/// Rotates the object around the Z axis by the specified amount (in radians)
		/// when drawing.
		pub fn rotated_z(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_z(rotation))
		}

		/// Sets the blend color to use when drawing.
		pub fn color(&self, color: impl Into<$crate::color::LinSrgba>) -> Self {
			let mut new = self.clone();
			new.color = color.into();
			new
		}

		/// Sets the blend mode to use when drawing.
		pub fn blend_mode(&self, blend_mode: $crate::graphics::BlendMode) -> Self {
			let mut new = self.clone();
			new.blend_mode = blend_mode;
			new
		}
	};
}
