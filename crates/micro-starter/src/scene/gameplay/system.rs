pub mod prelude;

use std::time::Duration;

use hecs::World;
use micro::{Context, Event};

use crate::globals::Globals;

use super::{context::GameplayContext, event::GameplayEvent};

#[allow(unused_variables)]
pub trait System {
	fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) {
	}

	fn ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &egui::Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) {
	}

	fn menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut egui::Ui,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) {
	}

	fn stats(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> Option<Vec<String>> {
		None
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		event: &Event,
	) {
	}

	fn gameplay_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		event: &GameplayEvent,
	) {
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		delta_time: Duration,
	) {
	}

	fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) {
	}
}
