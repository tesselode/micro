use micro::{
	Context,
	color::{ColorConstants, LinSrgba},
	graphics::texture::Texture,
	math::{Vec2, vec2},
};

use super::{LayoutResult, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Image {
	texture: Texture,
	scale: Vec2,
	color: LinSrgba,
	mouse_state: Option<WidgetMouseState>,
}

impl Image {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			scale: Vec2::ONE,
			color: LinSrgba::WHITE,
			mouse_state: None,
		}
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn scale(self, scale: impl Into<Vec2>) -> Self {
		Self {
			scale: scale.into(),
			..self
		}
	}

	pub fn scale_x(self, scale_x: f32) -> Self {
		Self {
			scale: vec2(scale_x, self.scale.y),
			..self
		}
	}

	pub fn scale_y(self, scale_y: f32) -> Self {
		Self {
			scale: vec2(self.scale.x, scale_y),
			..self
		}
	}

	pub fn uniform_scale(self, scale: f32) -> Self {
		Self {
			scale: Vec2::splat(scale),
			..self
		}
	}

	pub fn mouse_state(self, state: &WidgetMouseState) -> Self {
		Self {
			mouse_state: Some(state.clone()),
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

	fn mouse_state(&self) -> Option<WidgetMouseState> {
		self.mouse_state.clone()
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

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) {
		let _span = tracy_client::span!();
		self.texture
			.color(self.color)
			.scaled_2d(self.scale)
			.draw(ctx);
	}
}
