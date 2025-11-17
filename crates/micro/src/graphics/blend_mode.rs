use wgpu::{BlendComponent, BlendFactor, BlendOperation, BlendState};

/// Determines how the pixels written by a drawing operation interact with
/// the pixels already at the same place.
///
/// These are the same as the corresponding blend modes in
/// [LÖVE](https://love2d.org/wiki/BlendMode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum BlendMode {
	/// The opacity of what's drawn is determined by the alpha component.
	Alpha(BlendAlphaMode),
	/// Colors are added to what's already on the surface.
	Add(BlendAlphaMode),
	/// Colors are subtracted from what's already on the surface.
	Subtract(BlendAlphaMode),
	/// Colors are multiplied by what's already on the surface.
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

/// How the alpha component of a color affects the red, green,
/// and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum BlendAlphaMode {
	/// The RGB components are multiplied by the alpha component.
	#[default]
	AlphaMultiply,
	/// The RGB components are not multiplied by the alpha component.
	///
	/// This is used by default when drawing a [`Canvas`](crate::graphics::Canvas),
	/// as it already has transparency applied to the pixels.
	Premultiplied,
}
