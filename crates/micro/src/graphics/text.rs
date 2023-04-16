mod font;

pub use font::*;
pub use fontdue::layout::{HorizontalAlign, VerticalAlign, WrapStyle};

use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use glam::Vec2;

use crate::{context::Context, math::Rect, IntoOffsetAndCount};

use super::{
	draw_params::DrawParams,
	sprite_batch::{SpriteBatch, SpriteParams},
};

pub struct Text {
	pub(crate) sprite_batches: Vec<SpriteBatch>,
	pub(crate) bounds: Option<Rect>,
}

impl Text {
	pub fn new(ctx: &Context, font: &Font, text: &str, layout_settings: LayoutSettings) -> Self {
		Self::with_multiple_fonts(
			ctx,
			&[font],
			&[TextFragment {
				font_index: 0,
				text,
			}],
			layout_settings,
		)
	}

	pub fn with_multiple_fonts<'a>(
		ctx: &Context,
		fonts: &[&Font],
		text_fragments: impl IntoIterator<Item = &'a TextFragment<'a>>,
		layout_settings: LayoutSettings,
	) -> Self {
		let fontdue_fonts = fonts.iter().map(|font| &font.font).collect::<Vec<_>>();
		let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
		layout.reset(&layout_settings.into());
		for TextFragment { font_index, text } in text_fragments {
			layout.append(
				&fontdue_fonts,
				&TextStyle {
					text,
					px: fonts[*font_index].scale,
					font_index: *font_index,
					user_data: (),
				},
			);
		}
		Self::from_layout(layout, fonts, ctx)
	}

	pub fn num_glyphs(&self) -> usize {
		self.sprite_batches
			.iter()
			.map(|sprite_batch| sprite_batch.len())
			.sum()
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.bounds
	}

	pub fn draw(&self, ctx: &mut Context, params: impl Into<DrawParams>) {
		let params = params.into();
		for sprite_batch in &self.sprite_batches {
			sprite_batch.draw(ctx, params);
		}
	}

	pub fn draw_range(
		&self,
		ctx: &mut Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams>,
	) {
		if self.sprite_batches.len() != 1 {
			unimplemented!("draw_range is only implemented for text with exactly 1 font");
		}
		self.sprite_batches[0].draw_range(ctx, range, params);
	}

	fn from_layout(layout: Layout, fonts: &[&Font], ctx: &Context) -> Text {
		let glyphs = layout.glyphs();
		let mut sprite_batches = fonts
			.iter()
			.enumerate()
			.map(|(i, font)| {
				SpriteBatch::new(
					ctx,
					&font.texture,
					glyphs.iter().filter(|glyph| glyph.font_index == i).count(),
				)
			})
			.collect::<Vec<_>>();
		let mut bounds: Option<Rect> = None;
		for glyph in glyphs {
			if !glyph.char_data.rasterize() {
				continue;
			}
			let display_rect = Rect::from_top_left_and_size(
				Vec2::new(glyph.x, glyph.y),
				Vec2::new(glyph.width as f32, glyph.height as f32),
			);
			if let Some(bounds) = &mut bounds {
				*bounds = bounds.union(display_rect);
			} else {
				bounds = Some(display_rect);
			}
			let texture_rect = *fonts[glyph.font_index]
				.glyph_rects
				.get(&glyph.parent)
				.unwrap_or_else(|| panic!("No glyph rect for the character {}", glyph.parent));
			sprite_batches[glyph.font_index]
				.add_region(
					ctx,
					texture_rect,
					SpriteParams::new().position(Vec2::new(glyph.x, glyph.y)),
				)
				.expect("Not enough capacity in the sprite batch");
		}
		Self {
			sprite_batches,
			bounds,
		}
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct LayoutSettings {
	/// The top-left boundary of the text region.
	pub position: Vec2,
	/// An optional rightmost boundary on the text region. A line of text that exceeds the
	/// max_width is wrapped to the line below. If the width of a glyph is larger than the
	/// max_width, the glyph will overflow past the max_width. The application is responsible for
	/// handling the overflow.
	pub max_width: Option<f32>,
	/// An optional bottom boundary on the text region. This is used for positioning the
	/// vertical_align option. Text that exceeds the defined max_height will overflow past it. The
	/// application is responsible for handling the overflow.
	pub max_height: Option<f32>,
	/// The default is Left. This option does nothing if the max_width isn't set.
	pub horizontal_align: HorizontalAlign,
	/// The default is Top. This option does nothing if the max_height isn't set.
	pub vertical_align: VerticalAlign,
	pub line_height: f32,
	/// The default is Word. Wrap style is a hint for how strings of text should be wrapped to the
	/// next line. Line wrapping can happen when the max width/height is reached.
	pub wrap_style: WrapStyle,
	/// The default is true. This option enables hard breaks, like new line characters, to
	/// prematurely wrap lines. If false, hard breaks will not prematurely create a new line.
	pub wrap_hard_breaks: bool,
}

impl Default for LayoutSettings {
	fn default() -> LayoutSettings {
		LayoutSettings {
			position: Vec2::ZERO,
			max_width: None,
			max_height: None,
			horizontal_align: HorizontalAlign::Left,
			vertical_align: VerticalAlign::Top,
			line_height: 1.0,
			wrap_style: WrapStyle::Word,
			wrap_hard_breaks: true,
		}
	}
}

impl From<LayoutSettings> for fontdue::layout::LayoutSettings {
	fn from(settings: LayoutSettings) -> Self {
		fontdue::layout::LayoutSettings {
			x: settings.position.x,
			y: settings.position.y,
			max_width: settings.max_width,
			max_height: settings.max_height,
			horizontal_align: settings.horizontal_align,
			vertical_align: settings.vertical_align,
			line_height: settings.line_height,
			wrap_style: settings.wrap_style,
			wrap_hard_breaks: settings.wrap_hard_breaks,
		}
	}
}

pub struct TextFragment<'a> {
	pub font_index: usize,
	pub text: &'a str,
}
