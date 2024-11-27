use std::{
	cell::RefCell,
	fmt::{Debug, Formatter},
};

use micro::{
	color::{ColorConstants, LinSrgba},
	graphics::text::{Font, HorizontalAlign, LayoutSettings, Text, VerticalAlign, WrapStyle},
	math::Vec2,
	Context,
};

use super::{LayoutResult, Widget, WidgetMouseEventChannel};

pub struct TextWidget {
	font: Font,
	text: String,
	settings: TextSettings,
	rendered: RefCell<Option<Text>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl TextWidget {
	pub fn new(font: &Font, text: impl Into<String>, settings: TextSettings) -> Self {
		Self {
			font: font.clone(),
			text: text.into(),
			settings,
			rendered: RefCell::new(None),
			mouse_event_channel: None,
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}
}

impl Widget for TextWidget {
	fn name(&self) -> &'static str {
		"text"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&[]
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
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
		let layout_settings = match self.settings.sizing {
			TextSizing::Min { .. } => LayoutSettings {
				line_height: self.settings.line_height,
				wrap_style: self.settings.wrap_style,
				wrap_hard_breaks: self.settings.wrap_hard_breaks,
				..Default::default()
			},
			TextSizing::Max {
				horizontal_align,
				vertical_align,
			} => LayoutSettings {
				max_width: Some(allotted_size_from_parent.x),
				max_height: Some(allotted_size_from_parent.y),
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
			TextSizing::Min {
				size_reporting: TextSizeReporting {
					include_lowest_line_descenders,
				},
			} => rendered
				.bounds()
				.map(|bounds| {
					let mut size = bounds.bottom_right();
					if !include_lowest_line_descenders {
						size.y = rendered.lowest_baseline().unwrap();
					}
					size
				})
				.unwrap_or_default(),
			TextSizing::Max { .. } => allotted_size_from_parent,
		};
		*self.rendered.borrow_mut() = Some(rendered);
		LayoutResult {
			size,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) -> anyhow::Result<()> {
		if let Some(TextShadow { color, offset }) = self.settings.shadow {
			self.rendered
				.borrow()
				.as_ref()
				.unwrap()
				.translated_2d(offset)
				.color(color)
				.draw(ctx);
		}
		self.rendered
			.borrow()
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

#[derive(Clone, Copy, PartialEq)]
pub enum TextSizing {
	Min {
		size_reporting: TextSizeReporting,
	},
	Max {
		horizontal_align: HorizontalAlign,
		vertical_align: VerticalAlign,
	},
}

impl Default for TextSizing {
	fn default() -> Self {
		Self::Min {
			size_reporting: TextSizeReporting::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextShadow {
	pub color: LinSrgba,
	pub offset: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextSizeReporting {
	pub include_lowest_line_descenders: bool,
}

impl Default for TextSizeReporting {
	fn default() -> Self {
		Self {
			include_lowest_line_descenders: true,
		}
	}
}
