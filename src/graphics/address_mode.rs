use wgpu::SamplerBorderColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressMode {
	Repeat,
	MirrorRepeat,
	ClampToEdge,
	ClampToBorder(SamplerBorderColor),
}

impl AddressMode {
	pub(crate) fn to_wgpu_address_mode(self) -> wgpu::AddressMode {
		match self {
			AddressMode::Repeat => wgpu::AddressMode::Repeat,
			AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
			AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
			AddressMode::ClampToBorder(_) => wgpu::AddressMode::ClampToBorder,
		}
	}

	pub(crate) fn border_color(self) -> Option<SamplerBorderColor> {
		match self {
			AddressMode::ClampToBorder(border_color) => Some(border_color),
			_ => None,
		}
	}
}

impl Default for AddressMode {
	fn default() -> Self {
		Self::Repeat
	}
}
