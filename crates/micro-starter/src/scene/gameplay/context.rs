use std::collections::VecDeque;

use hecs::CommandBuffer;

use crate::scene_manager::SceneChange;

use super::event::GameplayEvent;

pub struct GameplayContext {
	pub world_command_buffer: CommandBuffer,
	pub event_queue: VecDeque<GameplayEvent>,
	pub scene_change: Option<SceneChange>,
}

impl GameplayContext {
	pub fn new() -> Self {
		Self {
			world_command_buffer: CommandBuffer::new(),
			event_queue: VecDeque::new(),
			scene_change: None,
		}
	}
}
