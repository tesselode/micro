use bytemuck::{Pod, Zeroable};

pub trait Shader: Clone {
	const SOURCE: &'static str;

	type Params: Pod + Zeroable;
}

#[derive(Clone)]
pub struct DefaultShader;

impl Shader for DefaultShader {
	const SOURCE: &'static str = include_str!("shader.wgsl");

	type Params = i32;
}
