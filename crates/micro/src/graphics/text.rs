//! Types related to drawing text.

mod builder;

pub use builder::*;

pub use cosmic_text::{
	Align as TextAlign, Stretch as TextStretch, Style as TextStyle, Weight as TextWeight,
};
use cosmic_text::{Attrs, Family, LetterSpacing, Metrics, Shaping};

use std::sync::Arc;

use glam::{Mat4, Vec2, vec2};
use palette::LinSrgba;

use crate::{
	Micro,
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
	pub fn new(micro: &mut Micro, builder: TextBuilder) -> Self {
		let _span = tracy_client::span!();
		let buffer = create_cosmic_text_buffer(micro, builder);
		let SpriteBatchWithInfo {
			sprite_batch,
			glyph_bounds,
			line_bounds,
		} = create_text_sprite_batch(micro, buffer);
		let num_glyphs = sprite_batch.len();
		Self {
			inner: Arc::new(TextInner {
				sprite_batch,
				glyph_bounds,
				line_bounds,
				num_glyphs: num_glyphs as u32,
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

	/// Returns a rectangle that surrounds all the glyphs in the text.
	///
	/// Returns `None` if there's no characters in this [`Text`].
	pub fn glyph_bounds(&self) -> Option<Rect> {
		self.inner.glyph_bounds
	}

	/// Returns a rectangle that surrounds all the lines of the text.
	/// Descenders in the bottom row of text are not included.
	///
	/// Returns `None` if there's no characters in this [`Text`].
	pub fn line_bounds(&self) -> Option<Rect> {
		self.inner.line_bounds
	}

	/// Draws the text.
	pub fn draw(&self, micro: &mut Micro) {
		let _span = tracy_client::span!();
		self.inner
			.sprite_batch
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.range(self.range)
			.draw(micro);
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

#[derive(Debug)]
struct TextInner {
	pub(crate) sprite_batch: SpriteBatch,
	pub(crate) glyph_bounds: Option<Rect>,
	pub(crate) line_bounds: Option<Rect>,
	pub(crate) num_glyphs: u32,
}

struct SpriteBatchWithInfo {
	sprite_batch: SpriteBatch,
	glyph_bounds: Option<Rect>,
	line_bounds: Option<Rect>,
}

fn create_cosmic_text_buffer(micro: &mut Micro, builder: TextBuilder) -> cosmic_text::Buffer {
	let mut buffer = cosmic_text::Buffer::new(
		&mut micro.text.font_system,
		Metrics::relative(builder.font_size, builder.line_height),
	);
	let (width, align) = match builder.horizontal_sizing {
		TextHorizontalSizing::Min => (None, None),
		TextHorizontalSizing::Fixed { width, align } => (Some(width), Some(align)),
	};
	buffer.set_text(
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
	buffer.set_size(width, None);
	buffer.shape_until_scroll(&mut micro.text.font_system, true);
	buffer
}

fn create_text_sprite_batch(micro: &mut Micro, buffer: cosmic_text::Buffer) -> SpriteBatchWithInfo {
	struct Sprite {
		position: Vec2,
		texture_rect: IRect,
	}

	let mut sprites: Vec<Sprite> = vec![];
	let mut glyph_bounds: Option<Rect> = None;
	let mut line_bounds: Option<Rect> = None;

	for run in buffer.layout_runs() {
		for glyph in run.glyphs {
			// create a sprite for this glyph
			let physical_glyph = glyph.physical((0.0, 0.0), 1.0);
			let Some(GlyphInfo {
				texture_rect,
				offset,
			}) = micro.text.glyph_rect(
				&micro.graphics.device,
				&micro.graphics.queue,
				physical_glyph.cache_key,
			)
			else {
				continue;
			};
			let position = vec2(
				physical_glyph.x as f32,
				run.line_y + physical_glyph.y as f32,
			) + offset.as_vec2();
			sprites.push(Sprite {
				position,
				texture_rect,
			});

			// update the total glyph bounds
			let individual_glyph_bounds = Rect::new(position, texture_rect.size.as_vec2());
			if let Some(glyph_bounds) = &mut glyph_bounds {
				*glyph_bounds = glyph_bounds.union(individual_glyph_bounds);
			} else {
				glyph_bounds = Some(individual_glyph_bounds);
			}
		}

		// update the total line bounds
		if let Some(first_character_x) = run.glyphs.first().map(|glyph| glyph.x) {
			let individual_line_bounds = Rect::new(
				vec2(first_character_x, run.line_top),
				vec2(run.line_w, run.line_y - run.line_top),
			);
			if let Some(line_bounds) = &mut line_bounds {
				*line_bounds = line_bounds.union(individual_line_bounds);
			} else {
				line_bounds = Some(individual_line_bounds);
			}
		}
	}

	// create sprite batch
	let mut sprite_batch = SpriteBatch::new(micro, &micro.text.texture, sprites.len());
	for Sprite {
		position,
		texture_rect,
	} in &sprites
	{
		sprite_batch
			.add_region(micro, texture_rect.as_rect(), *position)
			.expect("sprite batch is full");
	}

	SpriteBatchWithInfo {
		sprite_batch,
		glyph_bounds,
		line_bounds,
	}
}
