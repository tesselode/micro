use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use glow::{HasContext, NativeBuffer, NativeVertexArray};
use lyon::{
	lyon_tessellation::{
		BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
		StrokeVertex, TessellationError, VertexBuffers,
	},
	path::Path,
};
use thiserror::Error;

use crate::{
	context::Context, draw_params::DrawParams, error::GlError, rect::Rect, texture::Texture,
};

pub struct Mesh {
	raw_mesh: Rc<RawMesh>,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u32]) -> Result<Self, GlError> {
		Ok(Self {
			raw_mesh: Rc::new(RawMesh::new(ctx.gl.clone(), vertices, indices)?),
		})
	}

	pub fn rectangle(ctx: &Context, rect: Rect) -> Result<Self, GlError> {
		Self::new(
			ctx,
			&[
				Vertex {
					position: rect.bottom_right(),
					texture_coords: Vec2::new(1.0, 1.0),
				},
				Vertex {
					position: rect.top_right(),
					texture_coords: Vec2::new(1.0, 0.0),
				},
				Vertex {
					position: rect.top_left,
					texture_coords: Vec2::new(0.0, 0.0),
				},
				Vertex {
					position: rect.bottom_left(),
					texture_coords: Vec2::new(0.0, 1.0),
				},
			],
			&[0, 1, 3, 1, 2, 3],
		)
	}

	pub fn from_path_fill(
		ctx: &Context,
		path: Path,
		options: &FillOptions,
	) -> Result<Self, FromPathError> {
		let mut geometry = VertexBuffers::<Vertex, u32>::new();
		let mut tessellator = FillTessellator::new();
		tessellator
			.tessellate_path(
				&path,
				options,
				&mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vertex {
					position: Vec2::new(vertex.position().x, vertex.position().y),
					texture_coords: Vec2::ZERO,
				}),
			)
			.map_err(FromPathError::TessellationError)?;
		Self::new(ctx, &geometry.vertices, &geometry.indices)
			.map_err(|error| FromPathError::GlError(error.0))
	}

	pub fn from_path_stroke(
		ctx: &Context,
		path: Path,
		options: &StrokeOptions,
	) -> Result<Self, FromPathError> {
		let mut geometry = VertexBuffers::<Vertex, u32>::new();
		let mut tessellator = StrokeTessellator::new();
		tessellator
			.tessellate_path(
				&path,
				options,
				&mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| Vertex {
					position: Vec2::new(vertex.position().x, vertex.position().y),
					texture_coords: Vec2::ZERO,
				}),
			)
			.map_err(FromPathError::TessellationError)?;
		Self::new(ctx, &geometry.vertices, &geometry.indices)
			.map_err(|error| FromPathError::GlError(error.0))
	}

	pub fn set_vertex(&self, index: usize, vertex: Vertex) {
		let gl = &self.raw_mesh.gl;
		unsafe {
			gl.bind_vertex_array(Some(self.raw_mesh.vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.raw_mesh.vertex_buffer));
			gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(std::mem::size_of::<Vertex>() * index) as i32,
				bytemuck::cast_slice(&[vertex]),
			);
		}
	}

	pub fn draw(&self, ctx: &Context, params: impl Into<DrawParams>) {
		self.draw_inner(ctx, &ctx.default_texture, params.into());
	}

	pub fn draw_textured(&self, ctx: &Context, texture: &Texture, params: impl Into<DrawParams>) {
		self.draw_inner(ctx, texture, params.into());
	}

	fn draw_inner(&self, ctx: &Context, texture: &Texture, params: DrawParams) {
		let gl = &ctx.gl;
		unsafe {
			let shader = params.shader.as_ref().unwrap_or(&ctx.default_shader);
			shader
				.send_color(ctx, "blendColor", params.color)
				.expect("Shader does not have a blendColor uniform");
			shader
				.send_mat4(ctx, "globalTransform", ctx.global_transform)
				.expect("Shader does not have a globalTransform uniform");
			shader
				.send_mat4(ctx, "localTransform", params.transform)
				.expect("Shader does not have a localTransform uniform");
			gl.use_program(Some(shader.raw_shader.program));
			gl.bind_texture(glow::TEXTURE_2D, Some(texture.raw_texture.texture));
			gl.bind_vertex_array(Some(self.raw_mesh.vertex_array));
			params.blend_mode.apply(gl);
			gl.draw_elements(
				glow::TRIANGLES,
				self.raw_mesh.num_indices,
				glow::UNSIGNED_INT,
				0,
			);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
	pub position: Vec2,
	pub texture_coords: Vec2,
}

pub struct RawMesh {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
	num_indices: i32,
}

impl RawMesh {
	pub fn new(
		gl: Rc<glow::Context>,
		vertices: &[Vertex],
		indices: &[u32],
	) -> Result<Self, GlError> {
		let vertex_array = unsafe { gl.create_vertex_array() }.map_err(GlError)?;
		let vertex_buffer = unsafe { gl.create_buffer() }.map_err(GlError)?;
		let index_buffer = unsafe { gl.create_buffer() }.map_err(GlError)?;
		unsafe {
			gl.bind_vertex_array(Some(vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(vertices),
				glow::STATIC_DRAW,
			);
			gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
			gl.buffer_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				bytemuck::cast_slice(indices),
				glow::STATIC_DRAW,
			);
			gl.vertex_attrib_pointer_f32(
				0,
				2,
				glow::FLOAT,
				false,
				std::mem::size_of::<Vertex>() as i32,
				0,
			);
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(
				1,
				2,
				glow::FLOAT,
				false,
				std::mem::size_of::<Vertex>() as i32,
				2 * std::mem::size_of::<f32>() as i32,
			);
			gl.enable_vertex_attrib_array(1);
		}
		Ok(Self {
			gl,
			vertex_array,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len().try_into().expect("Mesh has too many indices"),
		})
	}
}

impl Drop for RawMesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
			self.gl.delete_buffer(self.index_buffer);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum FromPathError {
	#[error("An error occured while tessellating the path")]
	TessellationError(TessellationError),
	#[error("{0}")]
	GlError(String),
}
