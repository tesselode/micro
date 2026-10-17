use micro::{Micro, input::Gamepad};
use micro_asset::{Assets, TextureLoader};
use micro_virtual_controller::VirtualController;
use tracing::warn;

use crate::input::{Controls, Sticks, default_input_config};

type Input = VirtualController<Controls, Sticks>;

pub struct Globals {
	pub input: Input,
	pub textures: Assets<TextureLoader>,
}

impl Globals {
	pub fn new(micro: &mut Micro) -> Self {
		Self {
			input: Input::new(default_input_config(), first_connected_gamepad(micro)),
			textures: Assets::autoloaded(micro, "texture", TextureLoader::default()),
		}
	}
}

fn first_connected_gamepad(micro: &mut Micro) -> Option<Gamepad> {
	let first_gamepad_id = match micro.connected_gamepad_ids() {
		Ok(ids) => Some(ids),
		Err(err) => {
			warn!("error getting connected gamepad IDs: {}", err);
			None
		}
	}?
	.drain(..)
	.next()?;
	match micro.gamepad(first_gamepad_id) {
		Ok(gamepad) => Some(gamepad),
		Err(err) => {
			warn!("error getting gamepad: {}", err);
			None
		}
	}
}
