use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		AddressMode, DrawParams,
	},
	input::Scancode,
	math::Rect,
	window::WindowMode,
	Context, ContextSettings, Event, State,
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
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed(Scancode::Space) = event {
			ctx.set_window_mode(WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			});
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.canvas.draw_region(
			ctx,
			Rect::xywh(-100.0, -100.0, 2000.0, 2000.0),
			DrawParams::new()
				.scaled(Vec2::new(2.0, 1.0))
				.rotated(0.5)
				.translated(Vec2::splat(100.0)),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
