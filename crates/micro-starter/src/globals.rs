use micro::Context;
use micro_asset::{Assets, FontLoader, TextureLoader};
use micro_virtual_controller::VirtualController;

use crate::input::{Controls, Sticks, default_input_config};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Assets<TextureLoader>,
	pub fonts: Assets<FontLoader>,
}

impl Globals {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			input: Input::new(
				default_input_config(),
				ctx.gamepads()
					.expect("could not get gamepads")
					.drain(..)
					.next(),
			),
			textures: Assets::autoloaded(ctx, "texture", TextureLoader::default()),
			fonts: Assets::autoloaded(ctx, "font", FontLoader::default()),
		}
	}
}
