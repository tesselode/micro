use std::{collections::HashMap, error::Error};

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		DrawParams,
	},
	input::{
		virtual_controller::{
			AxisDirection, DeadzoneShape, RealControl, VirtualAnalogStickControls,
			VirtualAnalogSticks, VirtualController, VirtualControllerConfig, VirtualControls,
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
	Right,
	Up,
	Down,
}

impl VirtualControls for Controls {
	fn all() -> &'static [Self] {
		&[Self::Left, Self::Right, Self::Up, Self::Down]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Sticks {
	Move,
}

impl VirtualAnalogSticks<Controls> for Sticks {
	fn all() -> &'static [Self] {
		&[Self::Move]
	}

	fn controls(&self) -> VirtualAnalogStickControls<Controls> {
		match self {
			Sticks::Move => VirtualAnalogStickControls {
				left: Controls::Left,
				right: Controls::Right,
				up: Controls::Up,
				down: Controls::Down,
			},
		}
	}
}

struct MainState {
	controller: VirtualController<Controls, Sticks>,
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
						mapping.insert(
							Controls::Right,
							vec![
								RealControl::Key(Scancode::Right),
								RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Positive),
							],
						);
						mapping.insert(
							Controls::Up,
							vec![
								RealControl::Key(Scancode::Up),
								RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Negative),
							],
						);
						mapping.insert(
							Controls::Down,
							vec![
								RealControl::Key(Scancode::Down),
								RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Positive),
							],
						);
						mapping
					},
					deadzone: 0.5,
					deadzone_shape: DeadzoneShape::Square,
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
		let stick = self.controller.stick(Sticks::Move);
		println!("{}, {}", stick.raw_value, stick.value);
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
