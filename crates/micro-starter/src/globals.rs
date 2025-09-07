use micro::{Context, input::virtual_controller::VirtualController};
use micro_resource::{FontLoader, Resources, TextureLoader};

use crate::input::{Controls, Sticks, default_input_config};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Resources<TextureLoader>,
	pub fonts: Resources<FontLoader>,
}

impl Globals {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			input: Input::new(default_input_config(), ctx.gamepad(0)),
			textures: Resources::autoloaded(ctx, "texture", TextureLoader::default()),
			fonts: Resources::autoloaded(ctx, "font", FontLoader::default()),
		}
	}
}
