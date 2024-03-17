use std::sync::{Mutex, MutexGuard, OnceLock};

use micro::{
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
	pub fn new() -> Self {
		Self {
			input: Input::new(default_input_config(), Some(0)),
			textures: Resources::autoloaded("texture", TextureLoader::default()),
			fonts: Resources::autoloaded("font", FontLoader::default()),
		}
	}

	pub fn init(&mut self) {
		self.textures.load_all();
		self.fonts.load_all();
	}
}

pub fn globals() -> MutexGuard<'static, Globals> {
	static GLOBALS: OnceLock<Mutex<Globals>> = OnceLock::new();
	GLOBALS
		.get_or_init(|| Mutex::new(Globals::new()))
		.lock()
		.unwrap()
}
