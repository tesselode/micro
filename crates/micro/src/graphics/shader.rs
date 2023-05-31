use bytemuck::{Pod, Zeroable};

pub trait Shader: Clone {
	const SOURCE: &'static str;
	const NUM_TEXTURES: u32;

	type Params: Pod + Zeroable;
}

#[derive(Clone)]
pub struct DefaultShader;

impl Shader for DefaultShader {
	const SOURCE: &'static str = include_str!("shader.wgsl");
	const NUM_TEXTURES: u32 = 0;

	type Params = i32;
}
