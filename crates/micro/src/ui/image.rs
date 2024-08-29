use glam::Vec2;
use palette::LinSrgba;

use crate::{color::ColorConstants, graphics::texture::Texture, Context};

use super::{LayoutResult, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Image {
	texture: Texture,
	color: LinSrgba,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Image {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			color: LinSrgba::WHITE,
			mouse_event_channel: None,
		}
	}

	pub fn with_color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}
}

impl Widget for Image {
	fn name(&self) -> &'static str {
		"image"
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
		_ctx: &mut Context,
		_allotted_size_from_parent: Vec2,
		_child_sizes: &[Vec2],
	) -> LayoutResult {
		LayoutResult {
			size: self.texture.size().as_vec2(),
			child_positions: vec![],
		}
	}

	fn draw(&self, ctx: &mut Context, _size: Vec2) -> anyhow::Result<()> {
		self.texture.color(self.color).draw(ctx);
		Ok(())
	}
}
