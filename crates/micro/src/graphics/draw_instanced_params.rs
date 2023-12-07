use glam::Vec4;
use glow::{HasContext, NativeBuffer};

use super::{shader::Shader, BlendMode, InstanceParams};

#[derive(Debug, Clone, Default)]
pub struct DrawInstancedSettings<'a> {
	pub instances: Vec<InstanceParams>,
	pub shader: Option<&'a Shader>,
	pub blend_mode: BlendMode,
}

impl<'a> DrawInstancedSettings<'a> {
	pub fn new(instances: impl IntoIterator<Item = impl Into<InstanceParams>>) -> Self {
		Self {
			instances: instances
				.into_iter()
				.map(|instance| instance.into())
				.collect(),
			..Default::default()
		}
	}

	pub fn instances(self, instances: impl IntoIterator<Item = impl Into<InstanceParams>>) -> Self {
		Self {
			instances: instances
				.into_iter()
				.map(|instance| instance.into())
				.collect(),
			..self
		}
	}

	pub fn shader(self, shader: &'a Shader) -> Self {
		Self {
			shader: Some(shader),
			..self
		}
	}

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
	}

	pub(crate) fn instance_buffer(&self, gl: &glow::Context) -> NativeBuffer {
		unsafe {
			let buffer = gl.create_buffer().expect("error creating instance buffer");
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(&self.instances),
				glow::STATIC_DRAW,
			);
			// local transform
			gl.enable_vertex_attrib_array(4);
			gl.vertex_attrib_pointer_f32(
				4,
				4,
				glow::FLOAT,
				false,
				(4 * std::mem::size_of::<Vec4>()) as i32,
				0,
			);
			gl.enable_vertex_attrib_array(5);
			gl.vertex_attrib_pointer_f32(
				5,
				4,
				glow::FLOAT,
				false,
				(4 * std::mem::size_of::<Vec4>()) as i32,
				std::mem::size_of::<Vec4>() as i32,
			);
			gl.enable_vertex_attrib_array(6);
			gl.vertex_attrib_pointer_f32(
				6,
				4,
				glow::FLOAT,
				false,
				(4 * std::mem::size_of::<Vec4>()) as i32,
				2 * std::mem::size_of::<Vec4>() as i32,
			);
			gl.enable_vertex_attrib_array(7);
			gl.vertex_attrib_pointer_f32(
				7,
				4,
				glow::FLOAT,
				false,
				(4 * std::mem::size_of::<Vec4>()) as i32,
				3 * std::mem::size_of::<Vec4>() as i32,
			);
			gl.vertex_attrib_divisor(4, 1);
			gl.vertex_attrib_divisor(5, 1);
			gl.vertex_attrib_divisor(6, 1);
			gl.vertex_attrib_divisor(7, 1);
			buffer
		}
	}
}

impl<'a, T, P> From<T> for DrawInstancedSettings<'a>
where
	T: IntoIterator<Item = P>,
	P: Into<InstanceParams>,
{
	fn from(instances: T) -> Self {
		Self::new(instances)
	}
}
