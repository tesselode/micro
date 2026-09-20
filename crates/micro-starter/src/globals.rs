use micro::Context;
use micro_asset::{Assets, TextureLoader};
use micro_virtual_controller::VirtualController;

use crate::input::{Controls, Sticks, default_input_config};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Assets<TextureLoader>,
}

impl Globals {
	pub fn new(micro: &mut Context) -> Self {
		Self {
			input: Input::new(
				default_input_config(),
				micro.gamepads()
					.expect("could not get gamepads")
					.drain(..)
					.next(),
			),
			textures: Assets::autoloaded(micro, "texture", TextureLoader::default()),
		}
	}
}
