use std::{collections::HashMap, error::Error};

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		DrawParams,
	},
	input::{
		virtual_controller::{
			AxisDirection, RealControl, VirtualController, VirtualControllerConfig, VirtualControls,
		},
		Axis, Button, GameController, MouseButton, Scancode,
	},
	math::Rect,
	Context, ContextSettings, State,
};
use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Controls {
	Left,
}

impl VirtualControls for Controls {
	fn all() -> &'static [Self] {
		&[Self::Left]
	}
}

struct MainState {
	controller: VirtualController<Controls>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			controller: VirtualController::new(
				VirtualControllerConfig {
					control_mapping: {
						let mut mapping = HashMap::new();
						mapping.insert(
							Controls::Left,
							vec![
								RealControl::Key(Scancode::Left),
								RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Negative),
							],
						);
						mapping
					},
					deadzone: 0.5,
				},
				ctx.game_controller(0),
			),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(
		&mut self,
		ctx: &mut Context,
		delta_time: std::time::Duration,
	) -> Result<(), Box<dyn Error>> {
		self.controller.update(ctx);
		let control = self.controller.control(Controls::Left);
		if control.pressed {
			println!("pressed");
		}
		if control.released {
			println!("released");
		}
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
