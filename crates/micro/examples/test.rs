use std::error::Error;

use glam::UVec2;
use micro::{
	animation::AnimationData,
	graphics::{
		texture::{Texture, TextureSettings},
		ColorConstants, DrawParams,
	},
	Context, ContextSettings, State, WindowMode,
};
use palette::LinSrgba;

pub struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let animation_data = AnimationData::from_json("crates/micro/examples/player.json")?;
		dbg!(animation_data);
		Ok(Self {
			texture: Texture::from_file(
				ctx,
				"crates/micro/examples/wall.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.texture.draw(ctx, DrawParams::new().rotated(0.5));
		Ok(())
	}
}

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	);
}
