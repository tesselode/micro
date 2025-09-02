use wgpu::{BlendComponent, BlendFactor, BlendOperation, BlendState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
	Alpha(BlendAlphaMode),
	Add(BlendAlphaMode),
	Subtract(BlendAlphaMode),
	Multiply,
}

impl BlendMode {
	pub(crate) fn to_blend_state(self) -> BlendState {
		match self {
			BlendMode::Alpha(blend_alpha_mode) => BlendState {
				color: BlendComponent {
					src_factor: match blend_alpha_mode {
						BlendAlphaMode::AlphaMultiply => BlendFactor::SrcAlpha,
						BlendAlphaMode::Premultiplied => BlendFactor::One,
					},
					dst_factor: BlendFactor::OneMinusSrcAlpha,
					operation: BlendOperation::Add,
				},
				alpha: BlendComponent {
					src_factor: BlendFactor::One,
					dst_factor: BlendFactor::OneMinusSrcAlpha,
					operation: BlendOperation::Add,
				},
			},
			BlendMode::Add(blend_alpha_mode) => BlendState {
				color: BlendComponent {
					src_factor: match blend_alpha_mode {
						BlendAlphaMode::AlphaMultiply => BlendFactor::SrcAlpha,
						BlendAlphaMode::Premultiplied => BlendFactor::One,
					},
					dst_factor: BlendFactor::One,
					operation: BlendOperation::Add,
				},
				alpha: BlendComponent {
					src_factor: BlendFactor::Zero,
					dst_factor: BlendFactor::One,
					operation: BlendOperation::Add,
				},
			},
			BlendMode::Subtract(blend_alpha_mode) => BlendState {
				color: BlendComponent {
					src_factor: match blend_alpha_mode {
						BlendAlphaMode::AlphaMultiply => BlendFactor::SrcAlpha,
						BlendAlphaMode::Premultiplied => BlendFactor::One,
					},
					dst_factor: BlendFactor::One,
					operation: BlendOperation::ReverseSubtract,
				},
				alpha: BlendComponent {
					src_factor: BlendFactor::Zero,
					dst_factor: BlendFactor::One,
					operation: BlendOperation::ReverseSubtract,
				},
			},
			BlendMode::Multiply => BlendState {
				color: BlendComponent {
					src_factor: BlendFactor::Dst,
					dst_factor: BlendFactor::Zero,
					operation: BlendOperation::Add,
				},
				alpha: BlendComponent {
					src_factor: BlendFactor::DstAlpha,
					dst_factor: BlendFactor::Zero,
					operation: BlendOperation::Add,
				},
			},
		}
	}
}

impl Default for BlendMode {
	fn default() -> Self {
		Self::Alpha(BlendAlphaMode::AlphaMultiply)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlendAlphaMode {
	#[default]
	AlphaMultiply,
	Premultiplied,
}
