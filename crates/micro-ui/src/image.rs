use micro::{
	color::{ColorConstants, LinSrgba},
	graphics::texture::Texture,
	math::{vec2, Vec2},
	Context,
};

use super::{LayoutResult, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Image {
	texture: Texture,
	scale: Vec2,
	color: LinSrgba,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Image {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			scale: Vec2::ONE,
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

	pub fn with_scale(self, scale: impl Into<Vec2>) -> Self {
		Self {
			scale: scale.into(),
			..self
		}
	}

	pub fn with_scale_x(self, scale_x: f32) -> Self {
		Self {
			scale: vec2(scale_x, self.scale.y),
			..self
		}
	}

	pub fn with_scale_y(self, scale_y: f32) -> Self {
		Self {
			scale: vec2(self.scale.x, scale_y),
			..self
		}
	}

	pub fn with_uniform_scale(self, scale: f32) -> Self {
		Self {
			scale: Vec2::splat(scale),
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
			size: self.texture.size().as_vec2() * self.scale,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) -> anyhow::Result<()> {
		let _span = tracy_client::span!();
		self.texture
			.color(self.color)
			.scaled_2d(self.scale)
			.draw(ctx);
		Ok(())
	}
}
