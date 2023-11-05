use glam::{UVec2, Vec2};

use crate::Context;

use super::{Canvas, CanvasSettings, DrawParams};

#[derive(Debug)]
pub struct Scaler {
	canvas: Canvas,
	integer_scale: bool,
}

impl Scaler {
	pub fn new(ctx: &Context, size: UVec2, integer_scale: bool) -> Self {
		Self {
			canvas: Canvas::new(ctx, size, CanvasSettings::default()),
			integer_scale,
		}
	}

	pub fn draw<T>(&self, ctx: &mut Context, f: impl FnMut(&mut Context) -> T) -> T {
		let result = self.canvas.render_to(ctx, f);
		let max_horizontal_scale = ctx.window_size().x as f32 / self.canvas.size().x as f32;
		let max_vertical_scale = ctx.window_size().y as f32 / self.canvas.size().y as f32;
		let mut scale = max_horizontal_scale.min(max_vertical_scale);
		if self.integer_scale {
			scale = scale.floor();
		}
		self.canvas.draw(
			ctx,
			DrawParams::new()
				.translated(-self.canvas.size().as_vec2() / 2.0)
				.scaled(Vec2::splat(scale))
				.translated(ctx.window_size().as_vec2() / 2.0),
		);
		result
	}
}
