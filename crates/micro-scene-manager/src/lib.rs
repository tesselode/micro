use std::time::Duration;

use micro::{Context, Event};

#[allow(unused_variables)]
pub trait Scene<Globals, Error> {
	fn name(&self) -> &'static str;

	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange<Globals, Error>> {
		None
	}

	fn debug_stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		None
	}

	fn debug_menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut micro::egui::Ui,
		globals: &mut Globals,
	) -> Result<(), Error> {
		Ok(())
	}

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) -> Result<(), Error> {
		Ok(())
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: &Event,
	) -> Result<(), Error> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> Result<(), Error> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		Ok(())
	}

	fn pause(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		Ok(())
	}

	fn resume(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		Ok(())
	}

	fn leave(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		Ok(())
	}
}

pub struct SceneManager<Globals, Error> {
	scenes: Vec<Box<dyn Scene<Globals, Error>>>,
}

impl<Globals, Error> SceneManager<Globals, Error> {
	pub fn new(first_scene: impl Scene<Globals, Error> + 'static) -> Self {
		Self {
			scenes: vec![Box::new(first_scene)],
		}
	}

	pub fn debug_stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		self.current_scene().debug_stats(ctx, globals)
	}

	pub fn debug_menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut micro::egui::Ui,
		globals: &mut Globals,
	) -> Result<(), Error> {
		self.current_scene().debug_menu(ctx, ui, globals)
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) -> Result<(), Error> {
		self.current_scene().debug_ui(ctx, egui_ctx, globals)
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: Event,
	) -> Result<(), Error> {
		self.current_scene().event(ctx, globals, &event)
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> Result<(), Error> {
		self.current_scene().update(ctx, globals, delta_time)
	}

	pub fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
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

	fn current_scene(&mut self) -> &mut Box<dyn Scene<Globals, Error>> {
		self.scenes.last_mut().expect("no current scene")
	}

	fn apply_scene_change(
		&mut self,
		ctx: &mut Context,
		scene_change: SceneChange<Globals, Error>,
		globals: &mut Globals,
	) -> Result<(), Error> {
		match scene_change {
			SceneChange::Switch(scene) => {
				tracy_client::Client::running()
					.unwrap()
					.message(&format!("Switching to scene: {}", scene.name()), 0);
				self.current_scene().leave(ctx, globals)?;
				*self.current_scene() = scene;
			}
			SceneChange::Push(scene) => {
				tracy_client::Client::running()
					.unwrap()
					.message(&format!("Pushing scene: {}", scene.name()), 0);
				self.current_scene().pause(ctx, globals)?;
				self.scenes.push(scene);
			}
			SceneChange::Pop => {
				tracy_client::Client::running()
					.unwrap()
					.message("Popping scene", 0);
				self.current_scene().leave(ctx, globals)?;
				self.scenes.pop();
				if self.scenes.is_empty() {
					panic!("cannot pop the last scene");
				}
				self.current_scene().resume(ctx, globals)?;
			}
			SceneChange::PopAndSwitch(scene) => {
				tracy_client::Client::running().unwrap().message(
					&format!("Popping scene and switching to: {}", scene.name()),
					0,
				);
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

pub enum SceneChange<Globals, Error> {
	Switch(Box<dyn Scene<Globals, Error>>),
	Push(Box<dyn Scene<Globals, Error>>),
	Pop,
	PopAndSwitch(Box<dyn Scene<Globals, Error>>),
}

impl<Globals, Error> SceneChange<Globals, Error> {
	pub fn switch(scene: impl Scene<Globals, Error> + 'static) -> Self {
		Self::Switch(Box::new(scene))
	}

	pub fn push(scene: impl Scene<Globals, Error> + 'static) -> Self {
		Self::Push(Box::new(scene))
	}

	pub fn pop_and_switch(scene: impl Scene<Globals, Error> + 'static) -> Self {
		Self::PopAndSwitch(Box::new(scene))
	}
}
