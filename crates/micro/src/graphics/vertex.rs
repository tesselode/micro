use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use palette::LinSrgba;

use super::VertexAttributeKind;

pub trait Vertex: Copy + Pod + Zeroable {
	const ATTRIBUTE_KINDS: &'static [VertexAttributeKind];
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex2d {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: LinSrgba,
}

impl Vertex for Vertex2d {
	const ATTRIBUTE_KINDS: &'static [VertexAttributeKind] = &[
		VertexAttributeKind::Vec2,
		VertexAttributeKind::Vec2,
		VertexAttributeKind::Vec4,
	];
}
