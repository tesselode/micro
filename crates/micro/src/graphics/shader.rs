use bytemuck::Pod;

use super::{Vertex, Vertex2d};

pub trait Shader {
	const SOURCE: &'static str;
	const NUM_STORAGE_BUFFERS: u32 = 0;

	type Vertex: Vertex;
	type Params: Pod;
}

pub struct DefaultShader;

impl Shader for DefaultShader {
	const SOURCE: &'static str = include_str!("shader.glsl");

	type Vertex = Vertex2d;
	type Params = i32;
}
