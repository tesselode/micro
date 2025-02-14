use std::{
	rc::Rc,
	sync::{
		atomic::{AtomicU64, Ordering},
		Weak,
	},
};

use bytemuck::{Pod, Zeroable};
use glow::{HasContext, NativeBuffer};

use crate::Context;

use super::resource::{GraphicsResource, GraphicsResourceId};

#[derive(Debug)]
pub struct VertexAttributeBuffer {
	pub(crate) id: VertexAttributeBufferId,
	_weak: Weak<()>,
}

impl VertexAttributeBuffer {
	pub fn new<T: VertexAttributes>(ctx: &mut Context, data: &[T]) -> Self {
		let _span = tracy_client::span!();
		let gl = &ctx.graphics.gl;
		let buffer = unsafe {
			let buffer = gl
				.create_buffer()
				.expect("error creating vertex attribute buffer");
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(data),
				glow::STATIC_DRAW,
			);
			buffer
		};
		let (id, weak) = ctx
			.graphics
			.vertex_attribute_buffers
			.insert(RawVertexAttributeBuffer {
				gl: gl.clone(),
				buffer,
				attribute_kinds: T::ATTRIBUTE_KINDS.to_vec(),
				divisor: T::DIVISOR,
			});
		Self { id, _weak: weak }
	}
}

#[derive(Debug)]
pub(crate) struct RawVertexAttributeBuffer {
	gl: Rc<glow::Context>,
	pub buffer: NativeBuffer,
	pub attribute_kinds: Vec<VertexAttributeKind>,
	pub divisor: VertexAttributeDivisor,
}

impl Drop for RawVertexAttributeBuffer {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_buffer(self.buffer);
		}
	}
}

pub trait VertexAttributes: Copy + Pod + Zeroable {
	const ATTRIBUTE_KINDS: &'static [VertexAttributeKind];
	const DIVISOR: VertexAttributeDivisor;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttributeKind {
	F32,
	Vec2,
	Vec3,
	Vec4,
	Mat4,
}

impl VertexAttributeKind {
	pub(crate) fn num_locations(self) -> usize {
		match self {
			VertexAttributeKind::F32 => 1,
			VertexAttributeKind::Vec2 => 1,
			VertexAttributeKind::Vec3 => 1,
			VertexAttributeKind::Vec4 => 1,
			VertexAttributeKind::Mat4 => 4,
		}
	}

	pub(crate) fn size(self) -> usize {
		match self {
			VertexAttributeKind::F32 => 1,
			VertexAttributeKind::Vec2 => 2,
			VertexAttributeKind::Vec3 => 3,
			VertexAttributeKind::Vec4 => 4,
			VertexAttributeKind::Mat4 => 4,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttributeDivisor {
	PerVertex,
	PerNInstances(u32),
}

impl VertexAttributeDivisor {
	fn as_u32(self) -> u32 {
		match self {
			VertexAttributeDivisor::PerVertex => 0,
			VertexAttributeDivisor::PerNInstances(n) => n,
		}
	}
}

pub(crate) fn configure_vertex_attributes_for_buffer(
	gl: &glow::Context,
	buffer: NativeBuffer,
	attribute_kinds: &[VertexAttributeKind],
	divisor: VertexAttributeDivisor,
	mut next_attribute_index: u32,
) -> u32 {
	let num_f32s = attribute_kinds
		.iter()
		.map(|attribute| attribute.num_locations() * attribute.size())
		.sum::<usize>();
	let stride = num_f32s * std::mem::size_of::<f32>();
	unsafe {
		gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
		let mut next_attribute_offset = 0;
		for attribute in attribute_kinds {
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
				gl.vertex_attrib_divisor(next_attribute_index, divisor.as_u32());
				next_attribute_index += 1;
				next_attribute_offset += attribute.size() * std::mem::size_of::<f32>();
			}
		}
	}
	next_attribute_index
}

impl GraphicsResource for RawVertexAttributeBuffer {
	type Id = VertexAttributeBufferId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VertexAttributeBufferId(pub u64);

static NEXT_VERTEX_ATTRIBUTE_BUFFER_ID: AtomicU64 = AtomicU64::new(0);

impl GraphicsResourceId for VertexAttributeBufferId {
	fn next() -> Self {
		VertexAttributeBufferId(NEXT_VERTEX_ATTRIBUTE_BUFFER_ID.fetch_add(1, Ordering::SeqCst))
	}
}
