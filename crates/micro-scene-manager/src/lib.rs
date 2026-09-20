use std::time::Duration;

use micro::{Micro, Event};

#[allow(unused_variables)]
pub trait Scene<Globals> {
	fn name(&self) -> &'static str;

	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange<Globals>> {
		None
	}

	fn debug_stats(&mut self, micro: &mut Micro, globals: &mut Globals) -> Option<Vec<String>> {
		None
	}

	fn debug_menu(&mut self, micro: &mut Micro, ui: &mut micro::egui::Ui, globals: &mut Globals) {}

	fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
	}

	fn event(&mut self, micro: &mut Micro, globals: &mut Globals, event: &Event) {}

	fn update(&mut self, micro: &mut Micro, globals: &mut Globals, delta_time: Duration) {}

	fn draw(&mut self, micro: &mut Micro, globals: &mut Globals) {}

	fn post_draw(&mut self, micro: &mut Micro, globals: &mut Globals) {}

	fn pause(&mut self, micro: &mut Micro, globals: &mut Globals) {}

	fn resume(&mut self, micro: &mut Micro, globals: &mut Globals) {}

	fn leave(&mut self, micro: &mut Micro, globals: &mut Globals) {}
}

pub struct SceneManager<Globals> {
	scenes: Vec<Box<dyn Scene<Globals>>>,
}

impl<Globals> SceneManager<Globals> {
	pub fn new(first_scene: impl Scene<Globals> + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn debug_stats(&mut self, micro: &mut Micro, globals: &mut Globals) -> Option<Vec<String>> {
		self.current_scene().debug_stats(micro, globals)
	}

	pub fn debug_menu(
		&mut self,
		micro: &mut Micro,
		ui: &mut micro::egui::Ui,
		globals: &mut Globals,
	) {
		self.current_scene().debug_menu(micro, ui, globals)
	}

	pub fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
		self.current_scene().debug_ui(micro, egui_ctx, globals)
	}

	pub fn event(&mut self, micro: &mut Micro, globals: &mut Globals, event: Event) {
		self.current_scene().event(micro, globals, &event)
	}

	pub fn update(&mut self, micro: &mut Micro, globals: &mut Globals, delta_time: Duration) {
		self.current_scene().update(micro, globals, delta_time)
	}

	pub fn draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		let mut first_scene_to_draw_index = self.scenes.len() - 1;
		while first_scene_to_draw_index > 0 && self.scenes[first_scene_to_draw_index].transparent()
		{
			first_scene_to_draw_index -= 1;
		}
		for i in first_scene_to_draw_index..self.scenes.len() {
			self.scenes[i].draw(micro, globals);
		}
		if let Some(scene_change) = self.current_scene().scene_change() {
			self.apply_scene_change(micro, scene_change, globals);
		}
	}

	pub fn post_draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.current_scene().post_draw(micro, globals)
	}

	fn current_scene(&mut self) -> &mut Box<dyn Scene<Globals>> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(
		&mut self,
		micro: &mut Micro,
		scene_change: SceneChange<Globals>,
		globals: &mut Globals,
	) {
		match scene_change {
			SceneChange::Switch(scene) => {
				tracy_client::Client::running()
					.unwrap()
					.message(&format!("Switching to scene: {}", scene.name()), 0);
				self.current_scene().leave(micro, globals);
				*self.current_scene() = scene;
			}
			SceneChange::Push(scene) => {
				tracy_client::Client::running()
					.unwrap()
					.message(&format!("Pushing scene: {}", scene.name()), 0);
				self.current_scene().pause(micro, globals);
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				tracy_client::Client::running()
					.unwrap()
					.message("Popping scene", 0);
				self.current_scene().leave(micro, globals);
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().resume(micro, globals);
			}
			SceneChange::PopAndSwitch(scene) => {
				tracy_client::Client::running().unwrap().message(
					&format!("Popping scene and switching to: {}", scene.name()),
					0,
				);
				self.current_scene().leave(micro, globals);
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().leave(micro, globals);
				*self.current_scene() = scene;
			}
		}
	}
}

pub enum SceneChange<Globals> {
	Switch(Box<dyn Scene<Globals>>),
	Push(Box<dyn Scene<Globals>>),
	Pop,
	PopAndSwitch(Box<dyn Scene<Globals>>),
}

impl<Globals> SceneChange<Globals> {
	pub fn switch(scene: impl Scene<Globals> + 'static) -> Self {
		Self::Switch(Box::new(scene))
	}

	pub fn push(scene: impl Scene<Globals> + 'static) -> Self {
		Self::Push(Box::new(scene))
	}

	pub fn pop_and_switch(scene: impl Scene<Globals> + 'static) -> Self {
		Self::PopAndSwitch(Box::new(scene))
	}
}
