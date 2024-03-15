mod blend_mode;
mod camera_3d;
mod canvas;
mod color_constants;
mod culling;
pub mod mesh;
mod nine_slice;
pub mod shader;
pub mod sprite_batch;
mod stencil;
pub mod text;
pub mod texture;
pub(crate) mod unused_resource;
mod vertex;
mod vertex_attributes;

pub use blend_mode::*;
pub use camera_3d::*;
pub use canvas::*;
pub use color_constants::*;
pub use culling::*;
pub use nine_slice::*;
pub use stencil::*;
pub use vertex::*;
pub use vertex_attributes::*;

pub use sdl2::video::SwapInterval;

macro_rules! standard_draw_command_methods {
	() => {
		pub fn shader(
			mut self,
			shader: impl Into<Option<&'a crate::graphics::shader::Shader>>,
		) -> Self {
			self.params.shader = shader.into();
			self
		}

		pub fn transformed(mut self, transform: impl Into<glam::Mat4>) -> Self {
			self.params.transform = transform.into() * self.params.transform;
			self
		}

		pub fn translated_2d(self, translation: impl Into<glam::Vec2>) -> Self {
			self.transformed(glam::Mat4::from_translation(translation.into().extend(0.0)))
		}

		pub fn translated_3d(self, translation: impl Into<glam::Vec3>) -> Self {
			self.transformed(glam::Mat4::from_translation(translation.into()))
		}

		pub fn scaled_2d(self, scale: impl Into<glam::Vec2>) -> Self {
			self.transformed(glam::Mat4::from_scale(scale.into().extend(1.0)))
		}

		pub fn scaled_3d(self, scale: impl Into<glam::Vec3>) -> Self {
			self.transformed(glam::Mat4::from_scale(scale.into()))
		}

		pub fn rotated_x(self, rotation: f32) -> Self {
			self.transformed(glam::Mat4::from_rotation_x(rotation))
		}

		pub fn rotated_y(self, rotation: f32) -> Self {
			self.transformed(glam::Mat4::from_rotation_y(rotation))
		}

		pub fn rotated_z(self, rotation: f32) -> Self {
			self.transformed(glam::Mat4::from_rotation_z(rotation))
		}

		pub fn color(mut self, color: impl Into<palette::LinSrgba>) -> Self {
			self.params.color = color.into();
			self
		}

		pub fn blend_mode(mut self, blend_mode: crate::graphics::BlendMode) -> Self {
			self.params.blend_mode = blend_mode;
			self
		}
	};
}

pub(crate) use standard_draw_command_methods;
