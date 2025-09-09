use std::{any::TypeId, borrow::Cow, collections::HashMap};

use wgpu::{
	ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
	FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayout, PrimitiveState,
	RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource,
	TextureFormat, VertexAttribute, VertexBufferLayout, VertexState, VertexStepMode,
	naga::ShaderStage,
};

use crate::{
	context::graphics::DrawCommand,
	graphics::{BlendMode, Vertex},
};

pub(super) struct CachedResources {
	pub(super) vertex_info: HashMap<TypeId, VertexInfo>,
	pub(super) shaders: HashMap<String, ShaderModulePair>,
	pub(super) render_pipelines: HashMap<RenderPipelineSettings, RenderPipeline>,
}

impl CachedResources {
	pub(super) fn new() -> Self {
		Self {
			vertex_info: HashMap::new(),
			shaders: HashMap::new(),
			render_pipelines: HashMap::new(),
		}
	}

	pub(super) fn cache_vertex_info<V: Vertex>(&mut self) {
		let vertex_type = TypeId::of::<V>();
		self.vertex_info
			.entry(vertex_type)
			.or_insert_with(|| VertexInfo::for_type::<V>());
	}

	pub(super) fn create_shaders(&mut self, device: &Device, draw_commands: &[DrawCommand]) {
		for DrawCommand {
			render_pipeline_settings,
			..
		} in draw_commands
		{
			self.shaders
				.entry(render_pipeline_settings.shader_source.clone())
				.or_insert_with(|| {
					ShaderModulePair::new(
						device,
						&render_pipeline_settings.shader_name,
						&render_pipeline_settings.shader_source,
					)
				});
		}
	}

	pub(super) fn create_render_pipelines(
		&mut self,
		device: &Device,
		pipeline_layout: &PipelineLayout,
		draw_commands: &[DrawCommand],
	) {
		for DrawCommand {
			render_pipeline_settings,
			..
		} in draw_commands
		{
			self.render_pipelines
				.entry(render_pipeline_settings.clone())
				.or_insert_with(|| {
					create_render_pipeline(
						device,
						&self.vertex_info,
						&self.shaders,
						render_pipeline_settings,
						pipeline_layout,
					)
				});
		}
	}
}

pub(super) struct VertexInfo {
	size: usize,
	attributes: Vec<VertexAttribute>,
}

impl VertexInfo {
	fn for_type<V: Vertex>() -> Self {
		Self {
			size: std::mem::size_of::<V>(),
			attributes: V::attributes(),
		}
	}
}

pub(super) struct ShaderModulePair {
	vertex: ShaderModule,
	fragment: ShaderModule,
}

impl ShaderModulePair {
	fn new(device: &Device, name: &str, source: &str) -> Self {
		let vertex = device.create_shader_module(ShaderModuleDescriptor {
			label: Some(&format!("{} - Vertex Shader", &name)),
			source: ShaderSource::Glsl {
				shader: Cow::Borrowed(source),
				stage: ShaderStage::Vertex,
				defines: &[("VERTEX", "1")],
			},
		});
		let fragment = device.create_shader_module(ShaderModuleDescriptor {
			label: Some(&format!("{} - Fragment Shader", &name)),
			source: ShaderSource::Glsl {
				shader: Cow::Borrowed(source),
				stage: ShaderStage::Fragment,
				defines: &[("FRAGMENT", "1")],
			},
		});
		Self { vertex, fragment }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct RenderPipelineSettings {
	pub(super) vertex_type: TypeId,
	pub(super) shader_name: String,
	pub(super) shader_source: String,
	pub(super) blend_mode: BlendMode,
	pub(super) enable_color_writes: bool,
	pub(super) enable_depth_testing: bool,
	pub(super) wgpu_stencil_state: wgpu::StencilState,
	pub(super) format: TextureFormat,
	pub(super) sample_count: u32,
}

fn create_render_pipeline(
	device: &Device,
	vertex_info: &HashMap<TypeId, VertexInfo>,
	shaders: &HashMap<String, ShaderModulePair>,
	settings: &RenderPipelineSettings,
	pipeline_layout: &PipelineLayout,
) -> RenderPipeline {
	let vertex_info = &vertex_info[&settings.vertex_type];
	device.create_render_pipeline(&RenderPipelineDescriptor {
		label: None,
		layout: Some(pipeline_layout),
		vertex: VertexState {
			module: &shaders[&settings.shader_source].vertex,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
			buffers: &[VertexBufferLayout {
				array_stride: vertex_info.size as u64,
				step_mode: VertexStepMode::Vertex,
				attributes: &vertex_info.attributes,
			}],
		},
		primitive: PrimitiveState::default(),
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth24PlusStencil8,
			depth_write_enabled: settings.enable_depth_testing,
			depth_compare: if settings.enable_depth_testing {
				CompareFunction::Less
			} else {
				CompareFunction::Always
			},
			stencil: settings.wgpu_stencil_state.clone(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState {
			count: settings.sample_count,
			..Default::default()
		},
		fragment: Some(FragmentState {
			module: &shaders[&settings.shader_source].fragment,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
			targets: &[Some(ColorTargetState {
				format: settings.format,
				blend: Some(settings.blend_mode.to_blend_state()),
				write_mask: if settings.enable_color_writes {
					ColorWrites::ALL
				} else {
					ColorWrites::empty()
				},
			})],
		}),
		multiview: None,
		cache: None,
	})
}
