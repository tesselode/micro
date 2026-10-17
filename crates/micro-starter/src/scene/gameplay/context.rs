use micro_scene_manager::SceneChange;

use crate::globals::Globals;

pub struct GameplayContext {
	pub scene_change: Option<SceneChange<Globals>>,
}

impl GameplayContext {
	pub fn new() -> Self {
		Self { scene_change: None }
	}
}
