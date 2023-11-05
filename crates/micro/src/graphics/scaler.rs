use glam::{Affine2, UVec2, Vec2};

use crate::Context;

use super::{Canvas, CanvasSettings};

#[derive(Debug)]
pub struct Scaler {
	mode: Mode,
}

impl Scaler {
	pub fn smooth(size: UVec2) -> Self {
		Self {
			mode: Mode::Smooth { base_size: size },
		}
	}

	pub fn pixelated(ctx: &Context, size: UVec2, integer_scale: bool) -> Self {
		Self {
			mode: Mode::Pixelated {
				canvas: Canvas::new(ctx, size, CanvasSettings::default()),
				integer_scale,
			},
		}
	}

	pub fn transform(&self, ctx: &Context) -> Affine2 {
		let (base_size, scale) = match &self.mode {
			Mode::Smooth { base_size } => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				(*base_size, scale)
			}
			Mode::Pixelated {
				canvas,
				integer_scale,
			} => {
				let max_horizontal_scale = ctx.window_size().x as f32 / canvas.size().x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / canvas.size().y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				(canvas.size(), scale)
			}
		};
		Affine2::from_translation(ctx.window_size().as_vec2() / 2.0)
			* Affine2::from_scale(Vec2::splat(scale))
			* Affine2::from_translation(-base_size.as_vec2() / 2.0)
	}

	pub fn draw<T>(&self, ctx: &mut Context, f: impl FnMut(&mut Context) -> T) -> T {
		match &self.mode {
			Mode::Smooth { .. } => ctx.with_transform(self.transform(ctx), f),
			Mode::Pixelated { canvas, .. } => {
				let result = canvas.render_to(ctx, f);
				canvas.draw(ctx, self.transform(ctx));
				result
			}
		}
	}
}

#[derive(Debug)]
enum Mode {
	Smooth { base_size: UVec2 },
	Pixelated { canvas: Canvas, integer_scale: bool },
}
