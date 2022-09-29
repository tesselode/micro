use fontdue::layout::{HorizontalAlign, VerticalAlign};
use glam::Vec2;

use crate::{
	graphics::{
		color::Rgba,
		text::{Font, LayoutSettings, Text as RawText},
		DrawParams,
	},
	Context,
};

use super::{BuiltWidget, Widget};

pub struct Text {
	pub font: Font,
	pub string: String,
	pub horizontal_align: HorizontalAlign,
	pub vertical_align: VerticalAlign,
	pub color: Rgba,
}

impl Text {
	pub fn new(font: &Font, text: &str) -> Self {
		Self {
			font: font.clone(),
			string: text.to_string(),
			horizontal_align: HorizontalAlign::Left,
			vertical_align: VerticalAlign::Top,
			color: Rgba::WHITE,
		}
	}

	pub fn with_horizontal_align(self, horizontal_align: HorizontalAlign) -> Self {
		Self {
			horizontal_align,
			..self
		}
	}

	pub fn with_vertical_align(self, vertical_align: VerticalAlign) -> Self {
		Self {
			vertical_align,
			..self
		}
	}

	pub fn with_color(self, color: Rgba) -> Self {
		Self { color, ..self }
	}
}

impl Widget for Text {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let text = RawText::new(
			ctx,
			&self.font,
			&self.string,
			LayoutSettings {
				max_width: Some(max_size.x),
				max_height: Some(max_size.y),
				horizontal_align: self.horizontal_align,
				vertical_align: self.vertical_align,
				..Default::default()
			},
		);
		let natural_size = text
			.bounds()
			.map(|bounds| bounds.bottom_right)
			.unwrap_or(Vec2::ZERO);
		let size = natural_size.min(max_size);
		let scale = size / natural_size;
		Box::new(BuiltText {
			size,
			scale,
			text,
			color: self.color,
		})
	}
}

struct BuiltText {
	size: Vec2,
	scale: Vec2,
	text: RawText,
	color: Rgba,
}

impl BuiltWidget for BuiltText {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.text
			.draw(ctx, DrawParams::new().scale(self.scale).color(self.color))
	}
}
