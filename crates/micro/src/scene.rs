use std::time::Duration;

use crate::Event;

#[allow(unused_variables)]
pub trait Scene<E> {
	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange<E>> {
		None
	}

	fn ui(&mut self, egui_ctx: &egui::Context) -> Result<(), E> {
		Ok(())
	}

	fn menu(&mut self, ui: &mut egui::Ui) -> Result<(), E> {
		Ok(())
	}

	fn stats(&mut self) -> Option<Vec<String>> {
		None
	}

	fn event(&mut self, event: &Event) -> Result<(), E> {
		Ok(())
	}

	fn update(&mut self, delta_time: Duration) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self) -> Result<(), E> {
		Ok(())
	}

	fn pause(&mut self) -> Result<(), E> {
		Ok(())
	}

	fn resume(&mut self) -> Result<(), E> {
		Ok(())
	}
}

pub struct SceneManager<E> {
	scenes: Vec<Box<dyn Scene<E>>>,
}

impl<E> SceneManager<E> {
	pub fn new(first_scene: impl Scene<E> + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn ui(&mut self, egui_ctx: &egui::Context) -> Result<(), E> {
		self.current_scene().ui(egui_ctx)
	}

	pub fn menu(&mut self, ui: &mut egui::Ui) -> Result<(), E> {
		self.current_scene().menu(ui)
	}

	pub fn stats(&mut self) -> Option<Vec<String>> {
		self.current_scene().stats()
	}

	pub fn event(&mut self, event: Event) -> Result<(), E> {
		self.current_scene().event(&event)
	}

	pub fn update(&mut self, delta_time: Duration) -> Result<(), E> {
		self.current_scene().update(delta_time)
	}

	pub fn draw(&mut self) -> Result<(), E> {
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

	fn current_scene(&mut self) -> &mut Box<dyn Scene<E>> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(&mut self, scene_change: SceneChange<E>) -> Result<(), E> {
		match scene_change {
			SceneChange::Switch(scene) => *self.current_scene() = scene,
			SceneChange::Push(scene) => {
				self.current_scene().pause()?;
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().resume()?;
			}
			SceneChange::PopAndSwitch(scene) => {
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				*self.current_scene() = scene;
			}
		}
		Ok(())
	}
}

pub enum SceneChange<E> {
	Switch(Box<dyn Scene<E>>),
	Push(Box<dyn Scene<E>>),
	Pop,
	PopAndSwitch(Box<dyn Scene<E>>),
}

impl<E> SceneChange<E> {
	pub fn switch(scene: impl Scene<E> + 'static) -> Self {
		Self::Switch(Box::new(scene))
	}

	pub fn push(scene: impl Scene<E> + 'static) -> Self {
		Self::Push(Box::new(scene))
	}

	pub fn pop_and_switch(scene: impl Scene<E> + 'static) -> Self {
		Self::PopAndSwitch(Box::new(scene))
	}
}
