use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		DrawParams,
	},
	input::{
		virtual_controller::{AxisDirection, RealControl},
		Axis, Button, GameController, MouseButton, Scancode,
	},
	math::Rect,
	Context, ContextSettings, State,
};
use vek::Vec2;

struct MainState {
	controller: Option<GameController>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			controller: ctx.controller(0),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(
		&mut self,
		ctx: &mut Context,
		delta_time: std::time::Duration,
	) -> Result<(), Box<dyn Error>> {
		println!(
			"{}",
			RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Positive)
				.value(ctx, self.controller.as_ref())
		);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Context::new(ContextSettings {
		window_size: Vec2::new(1280, 720),
		vsync: false,
		..Default::default()
	})?
	.run(MainState::new)?;
	Ok(())
}
