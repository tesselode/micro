pub use wgpu::{ShaderModuleDescriptor, include_wgsl};

use bytemuck::Pod;

use super::{Vertex, Vertex2d};

pub trait Shader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_>;
	const NUM_STORAGE_BUFFERS: u32 = 0;
	const NUM_TEXTURES: u32 = 0;

	type Vertex: Vertex;
	type Params: Pod;
}

pub struct DefaultShader;

impl Shader for DefaultShader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");

	type Vertex = Vertex2d;
	type Params = i32;
}
