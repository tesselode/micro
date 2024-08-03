use std::time::Duration;

use crate::{Context, Event};

#[allow(unused_variables)]
pub trait Scene<G, E> {
	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange<G, E>> {
		None
	}

	fn ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &crate::ui::Context,
		globals: &mut G,
	) -> Result<(), E> {
		Ok(())
	}

	fn menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut crate::ui::Ui,
		globals: &mut G,
	) -> Result<(), E> {
		Ok(())
	}

	fn stats(&mut self, ctx: &mut Context, globals: &mut G) -> Option<Vec<String>> {
		None
	}

	fn event(&mut self, ctx: &mut Context, globals: &mut G, event: &Event) -> Result<(), E> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut G,
		delta_time: Duration,
	) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut G) -> Result<(), E> {
		Ok(())
	}

	fn pause(&mut self, ctx: &mut Context, globals: &mut G) -> Result<(), E> {
		Ok(())
	}

	fn resume(&mut self, ctx: &mut Context, globals: &mut G) -> Result<(), E> {
		Ok(())
	}

	fn leave(&mut self, ctx: &mut Context, globals: &mut G) -> Result<(), E> {
		Ok(())
	}
}

pub struct SceneManager<G, E> {
	scenes: Vec<Box<dyn Scene<G, E>>>,
}

impl<G, E> SceneManager<G, E> {
	pub fn new(first_scene: impl Scene<G, E> + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &crate::ui::Context,
		globals: &mut G,
	) -> Result<(), E> {
		self.current_scene().ui(ctx, egui_ctx, globals)
	}

	pub fn menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut crate::ui::Ui,
		globals: &mut G,
	) -> Result<(), E> {
		self.current_scene().menu(ctx, ui, globals)
	}

	pub fn stats(&mut self, ctx: &mut Context, globals: &mut G) -> Option<Vec<String>> {
		self.current_scene().stats(ctx, globals)
	}

	pub fn event(&mut self, ctx: &mut Context, globals: &mut G, event: Event) -> Result<(), E> {
		self.current_scene().event(ctx, globals, &event)
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut G,
		delta_time: Duration,
	) -> Result<(), E> {
		self.current_scene().update(ctx, globals, delta_time)
	}

	pub fn draw(&mut self, ctx: &mut Context, globals: &mut G) -> Result<(), E> {
		let mut first_scene_to_draw_index = self.scenes.len() - 1;
		while first_scene_to_draw_index > 0 && self.scenes[first_scene_to_draw_index].transparent()
		{
			first_scene_to_draw_index -= 1;
		}
		for i in first_scene_to_draw_index..self.scenes.len() {
			self.scenes[i].draw(ctx, globals)?;
		}
		if let Some(scene_change) = self.current_scene().scene_change() {
			self.apply_scene_change(ctx, scene_change, globals)?;
		}
		Ok(())
	}

	fn current_scene(&mut self) -> &mut Box<dyn Scene<G, E>> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(
		&mut self,
		ctx: &mut Context,
		scene_change: SceneChange<G, E>,
		globals: &mut G,
	) -> Result<(), E> {
		match scene_change {
			SceneChange::Switch(scene) => {
				self.current_scene().leave(ctx, globals)?;
				*self.current_scene() = scene;
			}
			SceneChange::Push(scene) => {
				self.current_scene().pause(ctx, globals)?;
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				self.current_scene().leave(ctx, globals)?;
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().resume(ctx, globals)?;
			}
			SceneChange::PopAndSwitch(scene) => {
				self.current_scene().leave(ctx, globals)?;
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().leave(ctx, globals)?;
				*self.current_scene() = scene;
			}
		}
		Ok(())
	}
}

pub enum SceneChange<G, E> {
	Switch(Box<dyn Scene<G, E>>),
	Push(Box<dyn Scene<G, E>>),
	Pop,
	PopAndSwitch(Box<dyn Scene<G, E>>),
}

impl<G, E> SceneChange<G, E> {
	pub fn switch(scene: impl Scene<G, E> + 'static) -> Self {
		Self::Switch(Box::new(scene))
	}

	pub fn push(scene: impl Scene<G, E> + 'static) -> Self {
		Self::Push(Box::new(scene))
	}

	pub fn pop_and_switch(scene: impl Scene<G, E> + 'static) -> Self {
		Self::PopAndSwitch(Box::new(scene))
	}
}
