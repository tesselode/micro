use micro::{
	game_controller,
	input::virtual_controller::VirtualController,
	resource::{
		loader::{FontLoader, TextureLoader},
		Resources,
	},
};

use crate::input::{default_input_config, Controls, Sticks};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Resources<TextureLoader>,
	pub fonts: Resources<FontLoader>,
}

impl Globals {
	pub fn new() -> anyhow::Result<Self> {
		Ok(Self {
			input: Input::new(default_input_config(), game_controller(0)),
			textures: Resources::autoloaded("texture", TextureLoader::default()),
			fonts: Resources::autoloaded("font", FontLoader::default()),
		})
	}
}
