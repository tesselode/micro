use std::fmt::{Debug, Formatter};

use fontdue::layout::{HorizontalAlign, VerticalAlign, WrapStyle};
use glam::Vec2;
use palette::LinSrgba;

use crate::{
	color::ColorConstants,
	graphics::text::{Font, LayoutSettings, Text},
	Context,
};

use super::Widget;

pub struct TextWidget {
	font: Font,
	text: String,
	settings: TextSettings,
	rendered: Option<Text>,
}

impl TextWidget {
	pub fn new(font: &Font, text: impl Into<String>, settings: TextSettings) -> Self {
		Self {
			font: font.clone(),
			text: text.into(),
			settings,
			rendered: None,
		}
	}
}

impl Widget for TextWidget {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		let layout_settings = match self.settings.sizing {
			TextSizing::Min => LayoutSettings {
				line_height: self.settings.line_height,
				wrap_style: self.settings.wrap_style,
				wrap_hard_breaks: self.settings.wrap_hard_breaks,
				..Default::default()
			},
			TextSizing::Max {
				horizontal_align,
				vertical_align,
			} => LayoutSettings {
				max_width: Some(max_size.x),
				max_height: Some(max_size.y),
				horizontal_align,
				vertical_align,
				line_height: self.settings.line_height,
				wrap_style: self.settings.wrap_style,
				wrap_hard_breaks: self.settings.wrap_hard_breaks,
				..Default::default()
			},
		};
		let rendered = Text::new(ctx, &self.font, &self.text, layout_settings);
		let size = match self.settings.sizing {
			TextSizing::Min => rendered
				.bounds()
				.map(|bounds| bounds.bottom_right())
				.unwrap_or_default(),
			TextSizing::Max { .. } => max_size,
		};
		self.rendered = Some(rendered);
		size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		if let Some(TextShadow { color, offset }) = self.settings.shadow {
			self.rendered
				.as_ref()
				.unwrap()
				.translated_2d(offset)
				.color(color)
				.draw(ctx);
		}
		self.rendered
			.as_ref()
			.unwrap()
			.color(self.settings.color)
			.draw(ctx);
		Ok(())
	}
}

impl Debug for TextWidget {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TextWidget")
			.field("text", &self.text)
			.finish()
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct TextSettings {
	pub sizing: TextSizing,
	pub line_height: f32,
	/// The default is Word. Wrap style is a hint for how strings of text should be wrapped to the
	/// next line. Line wrapping can happen when the max width/height is reached.
	pub wrap_style: WrapStyle,
	/// The default is true. This option enables hard breaks, like new line characters, to
	/// prematurely wrap lines. If false, hard breaks will not prematurely create a new line.
	pub wrap_hard_breaks: bool,
	pub color: LinSrgba,
	pub shadow: Option<TextShadow>,
}

impl Default for TextSettings {
	fn default() -> Self {
		Self {
			sizing: Default::default(),
			line_height: 1.0,
			wrap_style: WrapStyle::Word,
			wrap_hard_breaks: true,
			color: LinSrgba::WHITE,
			shadow: None,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TextSizing {
	#[default]
	Min,
	Max {
		horizontal_align: HorizontalAlign,
		vertical_align: VerticalAlign,
	},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextShadow {
	pub color: LinSrgba,
	pub offset: Vec2,
}
