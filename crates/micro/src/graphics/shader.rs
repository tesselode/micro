use std::marker::PhantomData;

use wgpu::VertexAttribute;
pub use wgpu::{ShaderModuleDescriptor, include_wgsl};

use bytemuck::Pod;

use super::{HasVertexAttributes, Vertex, Vertex2d};

pub trait Shader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_>;

	type Kind: ShaderKind;
	type Vertex: Vertex;
	type Params: Pod;
}

pub struct DefaultShader;

impl Shader for DefaultShader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");

	type Kind = NonInstanced;
	type Vertex = Vertex2d;
	type Params = i32;
}

pub trait ShaderKind {
	fn instance_settings() -> Option<InstanceSettings>;
}

pub struct Instanced<T: Pod + HasVertexAttributes>(pub PhantomData<T>);

impl<T: Pod + HasVertexAttributes> ShaderKind for Instanced<T> {
	fn instance_settings() -> Option<InstanceSettings> {
		Some(InstanceSettings {
			array_stride: std::mem::size_of::<T>() as u64,
			attributes: T::attributes(),
		})
	}
}

pub struct NonInstanced;

impl ShaderKind for NonInstanced {
	fn instance_settings() -> Option<InstanceSettings> {
		None
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceSettings {
	pub array_stride: u64,
	pub attributes: Vec<VertexAttribute>,
}
