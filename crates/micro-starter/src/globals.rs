use micro::{input::virtual_controller::VirtualController, Context};

use crate::{
	input::{default_input_config, Controls, Sticks},
	resources::Resources,
};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub resources: Resources,
	pub input: Input,
}

impl Globals {
	pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
		Ok(Self {
			resources: Resources::new(ctx)?,
			input: Input::new(default_input_config(), ctx.game_controller(0)),
		})
	}
}
