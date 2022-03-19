use std::{error::Error, time::Duration};

use glam::{Mat4, Vec3};
use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, Text},
		texture::{Texture, TextureFilter, TextureSettings},
		DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};

struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::load(ctx, "examples/player.png", TextureSettings::default())?;
		Ok(Self { texture })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		println!("delta time: {}", delta_time.as_secs_f32());
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.texture
			.draw_region(ctx, Rect::xywh(0.0, 0.0, 76.0, 95.0), DrawParams::new())?;
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Context::new(ContextSettings {
		window_width: 1280,
		window_height: 720,
		vsync: false,
		..Default::default()
	})?
	.run(MainState::new)?;
	Ok(())
}
