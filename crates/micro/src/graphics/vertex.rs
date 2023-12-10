use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use glow::HasContext;
use palette::LinSrgba;

pub trait Vertex: Copy + Pod + Zeroable {
	const ATTRIBUTES: &'static [VertexAttribute];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttribute {
	F32,
	Vec2,
	Vec3,
	Vec4,
	Mat4,
}

impl VertexAttribute {
	pub(crate) fn num_locations(self) -> usize {
		match self {
			VertexAttribute::F32 => 1,
			VertexAttribute::Vec2 => 1,
			VertexAttribute::Vec3 => 1,
			VertexAttribute::Vec4 => 1,
			VertexAttribute::Mat4 => 4,
		}
	}

	pub(crate) fn size(self) -> usize {
		match self {
			VertexAttribute::F32 => 1,
			VertexAttribute::Vec2 => 2,
			VertexAttribute::Vec3 => 3,
			VertexAttribute::Vec4 => 4,
			VertexAttribute::Mat4 => 4,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex2d {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: LinSrgba,
}

impl Vertex for Vertex2d {
	const ATTRIBUTES: &'static [VertexAttribute] = &[
		VertexAttribute::Vec2,
		VertexAttribute::Vec2,
		VertexAttribute::Vec4,
	];
}

pub(crate) fn configure_vertex_attributes(gl: &glow::Context, attributes: &[VertexAttribute]) {
	let num_f32s = attributes
		.iter()
		.map(|attribute| attribute.num_locations() * attribute.size())
		.sum::<usize>();
	let stride = num_f32s * std::mem::size_of::<f32>();
	unsafe {
		let mut next_attribute_index = 0;
		let mut next_attribute_offset = 0;
		for attribute in attributes {
			for _ in 0..attribute.num_locations() {
				gl.enable_vertex_attrib_array(next_attribute_index);
				gl.vertex_attrib_pointer_f32(
					next_attribute_index,
					attribute.size() as i32,
					glow::FLOAT,
					false,
					stride as i32,
					next_attribute_offset as i32,
				);
				next_attribute_index += 1;
				next_attribute_offset += attribute.size() * std::mem::size_of::<f32>();
			}
		}
	}
}
