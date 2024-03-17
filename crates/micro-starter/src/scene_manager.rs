use std::time::Duration;

use anyhow::anyhow;
use micro::Event;

use crate::scene::Scene;

pub struct SceneManager {
	scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
	pub fn new(first_scene: impl Scene + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn ui(&mut self, egui_ctx: &egui::Context) -> anyhow::Result<()> {
		self.current_scene().ui(egui_ctx)
	}

	pub fn menu(&mut self, ui: &mut egui::Ui) -> anyhow::Result<()> {
		self.current_scene().menu(ui)
	}

	pub fn stats(&mut self) -> Option<Vec<String>> {
		self.current_scene().stats()
	}

	pub fn event(&mut self, event: Event) -> anyhow::Result<()> {
		self.current_scene().event(&event)
	}

	pub fn update(&mut self, delta_time: Duration) -> anyhow::Result<()> {
		self.current_scene().update(delta_time)
	}

	pub fn draw(&mut self) -> anyhow::Result<()> {
		let mut first_scene_to_draw_index = self.scenes.len() - 1;
		while first_scene_to_draw_index > 0 && self.scenes[first_scene_to_draw_index].transparent()
		{
			first_scene_to_draw_index -= 1;
		}
		for i in first_scene_to_draw_index..self.scenes.len() {
			self.scenes[i].draw()?;
		}
		if let Some(scene_change) = self.current_scene().scene_change() {
			self.apply_scene_change(scene_change)?;
		}
		Ok(())
	}

	fn current_scene(&mut self) -> &mut Box<dyn Scene> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(&mut self, scene_change: SceneChange) -> anyhow::Result<()> {
		match scene_change {
			SceneChange::Switch(scene) => *self.current_scene() = scene,
			SceneChange::Push(scene) => {
				self.current_scene().pause()?;
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					return Err(anyhow!("cannot pop the last scene"));
				}
				self.current_scene().resume()?;
			}
			SceneChange::PopAndSwitch(scene) => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					return Err(anyhow!("cannot pop the last scene"));
				}
				*self.current_scene() = scene;
			}
		}
		Ok(())
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
