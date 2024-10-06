use micro::{input::virtual_controller::VirtualController, Context};
use micro_resource::{FontLoader, Resources, TextureLoader};

use crate::input::{default_input_config, Controls, Sticks};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Resources<TextureLoader>,
	pub fonts: Resources<FontLoader>,
}

impl Globals {
	pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
		Ok(Self {
			input: Input::new(default_input_config(), ctx.gamepad(0)),
			textures: Resources::autoloaded(ctx, "texture", TextureLoader::default()),
			fonts: Resources::autoloaded(ctx, "font", FontLoader::default()),
		})
	}
}
