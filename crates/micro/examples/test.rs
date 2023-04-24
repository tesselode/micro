use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		AddressMode, DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};
use wgpu::FilterMode;

struct MainState {
	canvas: Canvas,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: {
				let canvas = Canvas::new(
					ctx,
					UVec2::splat(200),
					CanvasSettings {
						address_mode: AddressMode::MirrorRepeat,
						magnifying_filter: FilterMode::Nearest,
						..Default::default()
					},
				);
				canvas.render_to(
					ctx,
					RenderToCanvasSettings {
						clear_color: Some(Rgba::BLACK),
						clear_stencil_value: None,
					},
					|ctx| {
						Mesh::circle(ctx, ShapeStyle::Fill, Vec2::ZERO, 100.0)
							.draw(ctx, Vec2::ZERO);
					},
				);
				canvas
			},
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.canvas.draw_region(
			ctx,
			Rect::xywh(-100.0, -100.0, 300.0, 300.0),
			DrawParams::new()
				.position(Vec2::splat(100.0))
				.scale(Vec2::splat(3.0)),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
