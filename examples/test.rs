use std::{error::Error, time::Duration};

use glam::{Mat4, Vec2, Vec3};
use micro::{
	graphics::{
		color::Rgba,
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};
use sdl2::{
	event::Event,
	keyboard::{Keycode, Scancode},
	mouse::MouseButton,
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
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyDown {
			keycode: Some(Keycode::Q),
			..
		} = event
		{
			ctx.quit();
		}
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		if ctx.is_mouse_button_down(MouseButton::Left) {
			println!("left");
		}
		if ctx.is_mouse_button_down(MouseButton::Right) {
			println!("right");
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		ctx.push_transform(Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0)))?;
		let size = Vec2::new(76.0, 95.0);
		self.texture.draw_region(
			ctx,
			Rect::new(Vec2::ZERO, size),
			DrawParams::new()
				.origin(size / 2.0)
				.scale(Vec2::splat(2.0))
				.rotation(0.5)
				.position(Vec2::splat(100.0)),
		)?;
		ctx.pop_transform()?;
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
