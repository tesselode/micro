use std::sync::Arc;

use cosmic_text::{Stretch as TextStretch, Style as TextStyle, Weight as TextWeight};
use glam::Vec2;

use crate::{
	Micro,
	graphics::{
		sprite_batch::SpriteParams,
		text::{Text, TextHorizontalSizing},
	},
};

pub type GlyphTransformFunction = Arc<dyn Fn(GlyphTransformInfo, SpriteParams) -> SpriteParams>;

#[derive(Clone)]
pub struct TextBuilder {
	pub font_family: String,
	pub font_size: f32,
	pub text: String,
	pub line_height: f32,
	pub stretch: TextStretch,
	pub style: TextStyle,
	pub weight: TextWeight,
	pub letter_spacing: Option<f32>,
	pub horizontal_sizing: TextHorizontalSizing,
	pub glyph_transform: Option<GlyphTransformFunction>,
}

impl TextBuilder {
	pub fn new(font_family: impl Into<String>, font_size: f32, text: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			text: text.into(),
			font_size,
			line_height: 1.0,
			stretch: TextStretch::Normal,
			style: TextStyle::Normal,
			weight: TextWeight::NORMAL,
			letter_spacing: None,
			horizontal_sizing: TextHorizontalSizing::default(),
			glyph_transform: None,
		}
	}

	pub fn font_family(self, font_family: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			..self
		}
	}

	pub fn font_size(self, font_size: f32) -> Self {
		Self { font_size, ..self }
	}

	pub fn text(self, text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			..self
		}
	}

	pub fn line_height(self, line_height: f32) -> Self {
		Self {
			line_height,
			..self
		}
	}

	pub fn stretch(self, stretch: TextStretch) -> Self {
		Self { stretch, ..self }
	}

	pub fn style(self, style: TextStyle) -> Self {
		Self { style, ..self }
	}

	pub fn weight(self, weight: TextWeight) -> Self {
		Self { weight, ..self }
	}

	pub fn letter_spacing(self, letter_spacing: impl Into<Option<f32>>) -> Self {
		Self {
			letter_spacing: letter_spacing.into(),
			..self
		}
	}

	pub fn horizontal_sizing(self, horizontal_sizing: TextHorizontalSizing) -> Self {
		Self {
			horizontal_sizing,
			..self
		}
	}

	pub fn glyph_transform(
		self,
		glyph_transform: impl Fn(GlyphTransformInfo, SpriteParams) -> SpriteParams + 'static,
	) -> Self {
		Self {
			glyph_transform: Some(Arc::new(glyph_transform)),
			..self
		}
	}

	pub fn build(self, micro: &mut Micro) -> Text {
		Text::new(micro, self)
	}
}

impl std::fmt::Debug for TextBuilder {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TextBuilder")
			.field("font_family", &self.font_family)
			.field("font_size", &self.font_size)
			.field("text", &self.text)
			.field("line_height", &self.line_height)
			.field("stretch", &self.stretch)
			.field("style", &self.style)
			.field("weight", &self.weight)
			.field("letter_spacing", &self.letter_spacing)
			.field("horizontal_sizing", &self.horizontal_sizing)
			.finish()
	}
}

pub struct GlyphTransformInfo {
	// The index of the character in the text (including spaces).
	pub character_index: usize,
	// The index of the visible character in the text (not including spaces).
	pub glyph_index: usize,
	pub position: Vec2,
}
