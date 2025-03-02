use bytemuck::Pod;

use super::{Vertex, Vertex2d};

pub trait Shader {
	const SOURCE: &'static str;

	type Vertex: Vertex;
	type Params: Pod;
}

pub struct DefaultShader;

impl Shader for DefaultShader {
	const SOURCE: &'static str = include_str!("shader.wgsl");

	type Vertex = Vertex2d;
	type Params = i32;
}
