use std::time::Duration;

use micro::{Context, Event};

use crate::{globals::Globals, scene::Scene};

pub struct SceneManager {
	scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
	pub fn new(first_scene: impl Scene + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context, globals: &mut Globals) {
		self.current_scene().ui(ctx, egui_ctx, globals)
	}

	pub fn menu(&mut self, ctx: &mut Context, ui: &mut egui::Ui, globals: &mut Globals) {
		self.current_scene().menu(ctx, ui, globals)
	}

	pub fn stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		self.current_scene().stats(ctx, globals)
	}

	pub fn event(&mut self, ctx: &mut Context, globals: &mut Globals, event: Event) {
		self.current_scene().event(ctx, globals, &event)
	}

	pub fn update(&mut self, ctx: &mut Context, globals: &mut Globals, delta_time: Duration) {
		self.current_scene().update(ctx, globals, delta_time)
	}

	pub fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) {
		let mut first_scene_to_draw_index = self.scenes.len() - 1;
		while first_scene_to_draw_index > 0 && self.scenes[first_scene_to_draw_index].transparent()
		{
			first_scene_to_draw_index -= 1;
		}
		for i in first_scene_to_draw_index..self.scenes.len() {
			self.scenes[i].draw(ctx, globals);
		}
		if let Some(scene_change) = self.current_scene().scene_change() {
			self.apply_scene_change(scene_change, ctx, globals);
		}
	}

	fn current_scene(&mut self) -> &mut Box<dyn Scene> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(
		&mut self,
		scene_change: SceneChange,
		ctx: &mut Context,
		globals: &mut Globals,
	) {
		match scene_change {
			SceneChange::Switch(scene) => *self.current_scene() = scene,
			SceneChange::Push(scene) => {
				self.current_scene().pause(ctx, globals);
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene")
				}
				self.current_scene().resume(ctx, globals);
			}
			SceneChange::PopAndSwitch(scene) => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene")
				}
				*self.current_scene() = scene;
			}
		}
	}
}

pub enum SceneChange {
	Switch(Box<dyn Scene>),
	Push(Box<dyn Scene>),
	Pop,
	PopAndSwitch(Box<dyn Scene>),
}

impl SceneChange {
	pub fn switch(scene: impl Scene + 'static) -> Self {
		Self::Switch(Box::new(scene))
	}

	pub fn push(scene: impl Scene + 'static) -> Self {
		Self::Push(Box::new(scene))
	}

	pub fn pop_and_switch(scene: impl Scene + 'static) -> Self {
		Self::PopAndSwitch(Box::new(scene))
	}
}