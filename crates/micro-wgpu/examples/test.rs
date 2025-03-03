use std::error::Error;

use glam::{Vec2, vec2};
use micro_wgpu::{
	App, Context, ContextSettings, Event,
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, builder::ShapeStyle},
	},
	input::Scancode,
	math::{Circle, URect},
};
use wgpu::PresentMode;

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(
		ContextSettings {
			present_mode: PresentMode::AutoNoVsync,
			resizable: true,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro_wgpu::debug_ui::Context,
	) -> Result<(), Self::Error> {
		egui::TopBottomPanel::top("main_menu").show(egui_ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.label(format!("FPS: {}", ctx.fps()));
			});
		});
		egui::Window::new("Test").show(egui_ctx, |ui| {
			ui.label("Hello!");
		});
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		if let Event::KeyPressed {
			key: Scancode::Space,
			..
		} = event
		{
			ctx.set_present_mode(match ctx.present_mode() {
				PresentMode::AutoVsync => PresentMode::AutoNoVsync,
				_ => PresentMode::AutoVsync,
			});
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		Ok(())
	}
}
