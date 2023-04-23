use glam::UVec2;
use wgpu::{
	Device, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
	TextureView,
};

pub fn create_depth_stencil_texture_view(size: UVec2, device: &Device) -> TextureView {
	let texture_size = Extent3d {
		width: size.x,
		height: size.y,
		depth_or_array_layers: 1,
	};
	let texture = device.create_texture(&TextureDescriptor {
		size: texture_size,
		mip_level_count: 1,
		sample_count: 1,
		dimension: TextureDimension::D2,
		format: TextureFormat::Depth24PlusStencil8,
		usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
		label: Some("Texture"),
		view_formats: &[],
	});
	texture.create_view(&wgpu::TextureViewDescriptor::default())
}
