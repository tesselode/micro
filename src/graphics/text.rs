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
	pub(crate) sprite_batch: SpriteBatch,
	pub(crate) bounds: Option<Rect>,
}

impl Text {
	pub fn new(ctx: &Context, font: &Font, text: &str, layout_settings: LayoutSettings) -> Self {
		let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
		layout.reset(&layout_settings.into());
		layout.append(
			&[&font.font],
			&TextStyle {
				text,
				px: font.scale,
				font_index: 0,
				user_data: (),
			},
		);
		let glyphs = layout.glyphs();
		let mut sprite_batch = SpriteBatch::new(ctx, &font.texture, glyphs.len());
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
			let texture_rect = *font
				.glyph_rects
				.get(&glyph.parent)
				.unwrap_or_else(|| panic!("No glyph rect for the character {}", glyph.parent));
			sprite_batch
				.add_region(
					texture_rect,
					SpriteParams::new().position(Vec2::new(glyph.x, glyph.y)),
				)
				.expect("Not enough capacity in the sprite batch");
		}
		Self {
			sprite_batch,
			bounds,
		}
	}

	pub fn num_glyphs(&self) -> usize {
		self.sprite_batch.len()
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.bounds
	}

	pub fn draw<'a>(&self, ctx: &mut Context, params: impl Into<DrawParams<'a>>) {
		self.sprite_batch.draw(ctx, params);
	}

	pub fn draw_range<'a>(
		&self,
		ctx: &mut Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<'a>>,
	) {
		self.sprite_batch.draw_range(ctx, range, params);
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
			wrap_style: settings.wrap_style,
			wrap_hard_breaks: settings.wrap_hard_breaks,
		}
	}
}
