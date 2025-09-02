use glam::{uvec2, vec2};
use micro2::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		Canvas, CanvasSettings, RenderToCanvasSettings, StencilState,
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use palette::LinSrgba;
use wgpu::{CompareFunction, StencilOperation, TextureFormat};

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {
	canvas: Canvas,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		Self {
			canvas: Canvas::new(
				ctx,
				uvec2(100, 100),
				CanvasSettings {
					sample_count: 8,
					format: TextureFormat::Rgba16Float,
					..Default::default()
				},
			),
		}
	}
}

impl App for Test {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		{
			let ctx = &mut self.canvas.render_to(
				ctx,
				RenderToCanvasSettings {
					clear_color: Some(LinSrgba::BLUE),
					..Default::default()
				},
			);
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: vec2(50.0, 50.0),
					radius: 50.0,
				},
			)
			.unwrap()
			.draw(ctx);
		}
		self.canvas.draw(ctx);
		self.canvas.translated_2d((75.0, 75.0)).draw(ctx);
	}
}
