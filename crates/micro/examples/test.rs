use std::{collections::HashMap, error::Error, time::Duration};

use glam::Vec2;
use micro::{
	animation::{AnimationData, AnimationPlayer},
	graphics::{
		texture::{Texture, TextureSettings},
		ColorConstants, DrawParams,
	},
	resource::{loader::MultipleAnimationDataLoader, Resources},
	Context, ContextSettings, State,
};
use palette::LinSrgba;

struct MainState {
	texture: Texture,
	animations: Resources<MultipleAnimationDataLoader>,
	animation_player: AnimationPlayer,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let animations = Resources::autoloaded(ctx, "ppl", MultipleAnimationDataLoader);
		dbg!(&animations);
		let animation_player =
			AnimationPlayer::new(animations["animations"]["player"].clone(), "Walk");
		Ok(Self {
			texture: Texture::from_file(
				ctx,
				"resources/ppl/sheet.png",
				TextureSettings::default(),
			)?,
			animations,
			animation_player,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.animation_player.update(delta_time);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::WHITE);
		self.animation_player.draw(
			ctx,
			&self.texture,
			DrawParams::new().scaled(Vec2::splat(10.0)),
		);
		Ok(())
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
