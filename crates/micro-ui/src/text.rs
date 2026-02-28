use std::{cell::RefCell, fmt::Debug};

use micro::{
	Context,
	color::{ColorConstants, LinSrgba},
	graphics::text::{
		Text, TextAlign, TextBuilder, TextHorizontalSizing, TextStretch, TextStyle, TextWeight,
	},
	math::Vec2,
};

use crate::{
	AxisSizing, Sizing, WidgetInspector, common_functions, common_widget_trait_functions,
	sizing_functions,
};

use super::{LayoutResult, Widget, WidgetMouseState};

#[derive(Debug, Clone)]
pub struct TextWidget {
	builder: TextBuilder,
	sizing: Sizing,
	align: TextAlign,
	color: LinSrgba,
	shadow: Option<TextShadow>,
	size_reporting: TextSizeReporting,
	built: RefCell<Option<Text>>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl TextWidget {
	pub fn new(font_family: impl Into<String>, text: impl Into<String>) -> Self {
		Self {
			builder: TextBuilder::new(font_family, text),
			sizing: Sizing::SHRINK,
			align: TextAlign::Left,
			color: LinSrgba::WHITE,
			shadow: None,
			size_reporting: TextSizeReporting::default(),
			built: RefCell::new(None),
			mouse_state: None,
			inspector: None,
		}
	}

	pub fn align(self, align: TextAlign) -> Self {
		Self { align, ..self }
	}

	pub fn font_family(self, font_family: impl Into<String>) -> Self {
		Self {
			builder: self.builder.font_family(font_family),
			..self
		}
	}

	pub fn text(self, text: impl Into<String>) -> Self {
		Self {
			builder: self.builder.text(text),
			..self
		}
	}

	pub fn font_size(self, font_size: f32) -> Self {
		Self {
			builder: self.builder.font_size(font_size),
			..self
		}
	}

	pub fn line_height(self, line_height: f32) -> Self {
		Self {
			builder: self.builder.line_height(line_height),
			..self
		}
	}

	pub fn stretch(self, stretch: TextStretch) -> Self {
		Self {
			builder: self.builder.stretch(stretch),
			..self
		}
	}

	pub fn style(self, style: TextStyle) -> Self {
		Self {
			builder: self.builder.style(style),
			..self
		}
	}

	pub fn weight(self, weight: TextWeight) -> Self {
		Self {
			builder: self.builder.weight(weight),
			..self
		}
	}

	pub fn letter_spacing(self, letter_spacing: impl Into<Option<f32>>) -> Self {
		Self {
			builder: self.builder.letter_spacing(letter_spacing),
			..self
		}
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn shadow(self, shadow: impl Into<Option<TextShadow>>) -> Self {
		Self {
			shadow: shadow.into(),
			..self
		}
	}

	pub fn size_reporting(self, size_reporting: TextSizeReporting) -> Self {
		Self {
			size_reporting,
			..self
		}
	}

	common_functions!();
	sizing_functions!();
}

impl Widget for TextWidget {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"text"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&[]
	}

	fn allotted_size_for_next_child(
		&self,
		_allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		unreachable!()
	}

	fn layout(
		&self,
		ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		_child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		let builder = self
			.builder
			.clone()
			.horizontal_sizing(match self.sizing.horizontal {
				AxisSizing::Shrink => TextHorizontalSizing::Min,
				sizing => TextHorizontalSizing::Fixed {
					width: sizing.allotted_size_for_children(allotted_size_from_parent.x),
					align: self.align,
				},
			});
		let built = builder.build(ctx);
		let bounds = match self.size_reporting {
			TextSizeReporting::Line => built.line_bounds(),
			TextSizeReporting::Glyph => built.glyph_bounds(),
		};
		let size = bounds.map(|bounds| bounds.size).unwrap_or_default();
		*self.built.borrow_mut() = Some(built);
		LayoutResult {
			size,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) {
		let _span = tracy_client::span!();
		let borrow = self.built.borrow();
		let built = borrow.as_ref().unwrap();
		let bounds = match self.size_reporting {
			TextSizeReporting::Line => built.line_bounds(),
			TextSizeReporting::Glyph => built.glyph_bounds(),
		};
		let position = bounds.map(|bounds| -bounds.top_left).unwrap_or_default();
		if let Some(TextShadow { color, offset }) = self.shadow {
			self.built
				.borrow()
				.as_ref()
				.unwrap()
				.translated_2d(position + offset)
				.color(color)
				.draw(ctx);
		}
		self.built
			.borrow()
			.as_ref()
			.unwrap()
			.translated_2d(position)
			.color(self.color)
			.draw(ctx);
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextShadow {
	pub color: LinSrgba,
	pub offset: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextSizeReporting {
	#[default]
	Line,
	Glyph,
}
