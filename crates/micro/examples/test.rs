use std::{error::Error, time::Duration};

use glam::UVec2;
use micro::{
	animation::{AnimationData, AnimationPlayer},
	graphics::{
		texture::{Texture, TextureSettings},
		ColorConstants, DrawParams,
	},
	Context, ContextSettings, State, WindowMode,
};
use palette::LinSrgba;

pub struct MainState {
	animation_player: AnimationPlayer,
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let animation_data = AnimationData::from_file("crates/micro/examples/player.json")?;
		let animation_player = AnimationPlayer::new(animation_data, "Jump".to_string());
		Ok(Self {
			animation_player,
			texture: Texture::from_file(
				ctx,
				"crates/micro/examples/player.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.animation_player.update(delta_time);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.animation_player
			.draw(ctx, &self.texture, DrawParams::new());
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
