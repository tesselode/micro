use std::{error::Error, time::Duration};

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		Canvas, CanvasSettings, ColorConstants, DrawParams,
	},
	input::Scancode,
	math::Circle,
	time::{FixedTimestepProducer, FixedTimestepProducerSettings},
	Context, ContextSettings, Event, ScalingMode, State, WindowMode,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::splat(400),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	canvas: Canvas,
	mesh: Mesh,
	fixed_timestep_producer: FixedTimestepProducer,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		tracing::error!("test error");
		Ok(Self {
			canvas: Canvas::new(ctx, ctx.window_size(), CanvasSettings::default()),
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::splat(50.0),
					radius: 16.0,
				},
				LinSrgba::WHITE,
			)?,
			fixed_timestep_producer: FixedTimestepProducer::new(FixedTimestepProducerSettings {
				ticks_per_second: 120.0,
				..Default::default()
			}),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, ctx: &mut Context, event: micro::Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed {
			key: Scancode::Z, ..
		} = event
		{
			let mut buffer = vec![0; (self.canvas.size().x * self.canvas.size().y * 4) as usize];
			self.canvas.read(&mut buffer);
			dbg!(buffer);
		}

		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		dbg!(ctx.mouse_position());
		self.fixed_timestep_producer
			.run(delta_time, |delta_time| -> Result<(), Box<dyn Error>> {
				Ok(())
			})?;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.canvas.render_to(ctx, |ctx| {
			ctx.clear(LinSrgba::BLACK);
			self.mesh.draw(ctx, DrawParams::new());
		});
		self.canvas.draw(ctx, DrawParams::new());
		Ok(())
	}
}
