use std::{error::Error, time::Duration};

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		ColorConstants, DrawParams, Scaler,
	},
	input::Scancode,
	math::Circle,
	time::{FixedTimestepProducer, FixedTimestepProducerSettings},
	Context, ContextSettings, Event, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	scaler: Scaler,
	mesh: Mesh,
	fixed_timestep_producer: FixedTimestepProducer,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			scaler: Scaler::smooth(UVec2::splat(100)),
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
			std::thread::sleep(Duration::from_millis(1000));
		}
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.fixed_timestep_producer.run(
			delta_time,
			|delta_time| -> Result<(), Box<dyn Error>> {
				println!("{:?}", delta_time);
				Ok(())
			},
		)?;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.scaler.draw(ctx, |ctx| {
			self.mesh.draw(ctx, DrawParams::new());
		});
		Ok(())
	}
}
