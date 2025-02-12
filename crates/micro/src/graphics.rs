mod blend_mode;
mod camera_3d;
mod canvas;
mod culling;
pub(crate) mod gpu_span;
pub mod mesh;
mod nine_slice;
pub(crate) mod resource;
pub mod shader;
pub mod sprite_batch;
mod stencil;
pub mod text;
pub mod texture;
mod vertex;
mod vertex_attributes;

pub use blend_mode::*;
pub use camera_3d::*;
pub use canvas::*;
pub use culling::*;
pub use nine_slice::*;
pub use stencil::*;
pub use vertex::*;
pub use vertex_attributes::*;

pub use sdl2::video::SwapInterval;

#[macro_export]
macro_rules! standard_draw_param_methods {
	() => {
		pub fn shader<'a>(
			&self,
			shader: impl Into<Option<&'a $crate::graphics::shader::Shader>>,
		) -> Self {
			let mut new = self.clone();
			new.shader = shader.into().cloned();
			new
		}

		pub fn transformed(&self, transform: impl Into<$crate::math::Mat4>) -> Self {
			let mut new = self.clone();
			new.transform = transform.into() * self.transform;
			new
		}

		pub fn translated_2d(&self, translation: impl Into<$crate::math::Vec2>) -> Self {
			self.transformed(Mat4::from_translation(translation.into().extend(0.0)))
		}

		pub fn translated_3d(&self, translation: impl Into<$crate::math::Vec3>) -> Self {
			self.transformed(Mat4::from_translation(translation.into()))
		}

		pub fn translated_x(&self, translation: f32) -> Self {
			self.translated_2d($crate::math::Vec2::new(translation, 0.0))
		}

		pub fn translated_y(&self, translation: f32) -> Self {
			self.translated_2d($crate::math::Vec2::new(0.0, translation))
		}

		pub fn translated_z(&self, translation: f32) -> Self {
			self.translated_3d($crate::math::Vec3::new(0.0, 0.0, translation))
		}

		pub fn scaled_2d(&self, scale: impl Into<$crate::math::Vec2>) -> Self {
			self.transformed(Mat4::from_scale(scale.into().extend(1.0)))
		}

		pub fn scaled_3d(&self, scale: impl Into<$crate::math::Vec3>) -> Self {
			self.transformed(Mat4::from_scale(scale.into()))
		}

		pub fn scaled_x(&self, scale: f32) -> Self {
			self.scaled_2d($crate::math::vec2(scale, 1.0))
		}

		pub fn scaled_y(&self, scale: f32) -> Self {
			self.scaled_2d($crate::math::vec2(1.0, scale))
		}

		pub fn scaled_z(&self, scale: f32) -> Self {
			self.scaled_3d($crate::math::vec3(1.0, 1.0, scale))
		}

		pub fn rotated(&self, rotation: $crate::math::Quat) -> Self {
			self.transformed(Mat4::from_quat(rotation))
		}

		pub fn rotated_x(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_x(rotation))
		}

		pub fn rotated_y(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_y(rotation))
		}

		pub fn rotated_z(&self, rotation: f32) -> Self {
			self.transformed(Mat4::from_rotation_z(rotation))
		}

		pub fn color(&self, color: impl Into<$crate::color::LinSrgba>) -> Self {
			let mut new = self.clone();
			new.color = color.into();
			new
		}

		pub fn blend_mode(&self, blend_mode: $crate::graphics::BlendMode) -> Self {
			let mut new = self.clone();
			new.blend_mode = blend_mode;
			new
		}
	};
}

pub(crate) use standard_draw_param_methods;
