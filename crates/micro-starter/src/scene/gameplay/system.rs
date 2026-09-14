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
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_stats(
		&mut self,		
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> Option<Vec<String>> {
		None
	}

	fn debug_menu(
		&mut self,		
		ui: &mut micro::egui::Ui,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_ui(
		&mut self,		
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn event(
		&mut self,		
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		event: &Event,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn gameplay_event(
		&mut self,		
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		event: &GameplayEvent,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(
		&mut self,		
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(
		&mut self,		
		globals: &mut Globals,
		gameplay_ctx: &mut GameplayContext,
		world: &mut World,
	) -> anyhow::Result<()> {
		Ok(())
	}
}
