//! Types related to drawing text.

pub use cosmic_text::{
	Align as TextAlign, Stretch as TextStretch, Style as TextStyle, Weight as TextWeight,
};
use cosmic_text::{Attrs, Family, LetterSpacing, Metrics, Shaping};

use std::sync::Arc;

use glam::{Mat4, Vec2, vec2};
use palette::LinSrgba;

use crate::{
	Context,
	color::ColorConstants,
	graphics::BlendMode,
	math::{IRect, Rect},
	standard_draw_param_methods,
	text::GlyphInfo,
};

use super::{IntoIndexRange, sprite_batch::SpriteBatch};

/// A block of text rendered into a texture.
#[derive(Debug, Clone)]
pub struct Text {
	inner: Arc<TextInner>,

	// params
	/// The transform to use when drawing this text.
	pub transform: Mat4,
	/// The blend color to use when drawing this text.
	pub color: LinSrgba,
	/// The blend mode to use when drawing this text.
	pub blend_mode: BlendMode,
	/// The min and max character index to use when drawing this text.
	///
	/// Setting this results in a portion of the text being drawn.
	/// When `None`, all the characters are drawn.
	pub range: Option<(u32, u32)>,
}

impl Text {
	/// Creates a new [`Text`].
	pub fn new(ctx: &mut Context, builder: TextBuilder) -> Self {
		let _span = tracy_client::span!();
		let mut buffer = cosmic_text::Buffer::new(
			&mut ctx.text.font_system,
			Metrics::relative(builder.font_size, builder.line_height),
		);
		let (width, align) = match builder.horizontal_sizing {
			TextHorizontalSizing::Min => (None, None),
			TextHorizontalSizing::Fixed { width, align } => (Some(width), Some(align)),
		};
		buffer.set_text(
			&mut ctx.text.font_system,
			&builder.text,
			&Attrs {
				family: Family::Name(&builder.font_family),
				stretch: builder.stretch,
				style: builder.style,
				weight: builder.weight,
				letter_spacing_opt: builder.letter_spacing.map(LetterSpacing),
				..Attrs::new()
			},
			Shaping::Advanced,
			align,
		);
		buffer.set_size(&mut ctx.text.font_system, width, None);
		buffer.shape_until_scroll(&mut ctx.text.font_system, true);
		let mut sprites: Vec<(IRect, Vec2)> = vec![];
		let mut bounds: Option<Rect> = None;
		let mut lowest_baseline: Option<f32> = None;
		for run in buffer.layout_runs() {
			for glyph in run.glyphs {
				let physical_glyph = glyph.physical((0.0, 0.0), 1.0);
				let Some(GlyphInfo {
					texture_rect,
					offset,
				}) = ctx.text.glyph_rect(
					&ctx.graphics.device,
					&ctx.graphics.queue,
					physical_glyph.cache_key,
				)
				else {
					continue;
				};
				let position = vec2(
					physical_glyph.x as f32,
					run.line_top + physical_glyph.y as f32,
				) + offset.as_vec2();
				sprites.push((texture_rect, position));
				let glyph_bounds = Rect::new(position, texture_rect.size.as_vec2());
				if let Some(bounds) = &mut bounds {
					*bounds = bounds.union(glyph_bounds);
				} else {
					bounds = Some(glyph_bounds);
				}
				if let Some(lowest_baseline) = &mut lowest_baseline {
					*lowest_baseline = lowest_baseline.max(run.line_top);
				} else {
					lowest_baseline = Some(run.line_top);
				}
			}
		}
		let mut sprite_batch = SpriteBatch::new(ctx, &ctx.text.texture, sprites.len());
		for (texture_region, position) in &sprites {
			sprite_batch
				.add_region(ctx, texture_region.as_rect(), *position)
				.expect("sprite batch is full");
		}
		Self {
			inner: Arc::new(TextInner {
				sprite_batch,
				bounds,
				lowest_baseline,
				num_glyphs: sprites.len() as u32,
			}),
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			range: None,
		}
	}

	standard_draw_param_methods!();

	/// Sets the range of character indices used for drawing.
	///
	/// Setting this results in a portion of the text being drawn.
	/// When `None`, all the characters are drawn.
	pub fn range(&self, range: impl IntoIndexRange) -> Self {
		let mut new = self.clone();
		new.range = range.into_index_range(self.inner.num_glyphs);
		new
	}

	/// Returns the number of glyphs this text contains.
	pub fn num_glyphs(&self) -> u32 {
		self.inner.num_glyphs
	}

	/// Returns a rectangle that tightly hugs the text.
	///
	/// Returns `None` if there's no characters in this [`Text`].
	pub fn bounds(&self) -> Option<Rect> {
		self.inner.bounds
	}

	/// Returns the y position of the lowest baseline of any
	/// of the characters.
	///
	/// Returns `None` if there's no characters in this [`Text`].
	pub fn lowest_baseline(&self) -> Option<f32> {
		self.inner.lowest_baseline
	}

	/// Draws the text.
	pub fn draw(&self, ctx: &mut Context) {
		let _span = tracy_client::span!();
		self.inner
			.sprite_batch
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.range(self.range)
			.draw(ctx);
	}
}

#[derive(Debug)]
struct TextInner {
	pub(crate) sprite_batch: SpriteBatch,
	pub(crate) bounds: Option<Rect>,
	pub(crate) lowest_baseline: Option<f32>,
	pub(crate) num_glyphs: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBuilder {
	pub font_family: String,
	pub text: String,
	pub font_size: f32,
	pub line_height: f32,
	pub stretch: TextStretch,
	pub style: TextStyle,
	pub weight: TextWeight,
	pub letter_spacing: Option<f32>,
	pub horizontal_sizing: TextHorizontalSizing,
}

impl TextBuilder {
	pub fn new(font_family: impl Into<String>, text: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			text: text.into(),
			font_size: 16.0,
			line_height: 1.0,
			stretch: TextStretch::Normal,
			style: TextStyle::Normal,
			weight: TextWeight::NORMAL,
			letter_spacing: None,
			horizontal_sizing: TextHorizontalSizing::default(),
		}
	}

	pub fn font_family(self, font_family: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			..self
		}
	}

	pub fn text(self, text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			..self
		}
	}

	pub fn font_size(self, font_size: f32) -> Self {
		Self { font_size, ..self }
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

	pub fn build(self, ctx: &mut Context) -> Text {
		Text::new(ctx, self)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextHorizontalSizing {
	#[default]
	Min,
	Fixed {
		width: f32,
		align: TextAlign,
	},
}
