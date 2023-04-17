use bytemuck::{Pod, Zeroable};

pub trait Shader {
	const SOURCE: &'static str;

	type Params: Pod + Zeroable;
}

pub(crate) struct DefaultShader;

impl Shader for DefaultShader {
	const SOURCE: &'static str = include_str!("shader.wgsl");

	type Params = i32;
}
