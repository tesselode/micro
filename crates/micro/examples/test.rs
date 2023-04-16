use std::error::Error;

use glam::UVec2;
use micro::{window::WindowMode, ContextSettings, State};

struct MainState;

impl State<Box<dyn Error>> for MainState {}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			},
			..Default::default()
		},
		|_| Ok(MainState),
	)
}
