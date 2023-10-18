use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::DrawParams,
	resource::{loader::TextureLoader, Resources},
	Context, ContextSettings, State, WindowMode,
};

pub struct MainState {
	textures: Resources<TextureLoader>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let mut textures = Resources::new(TextureLoader::default());
		textures.load(ctx, "")?;
		dbg!(&textures);
		Ok(Self { textures })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.textures
			.get("player")
			.unwrap()
			.draw(ctx, DrawParams::new().scaled(Vec2::splat(2.0)));
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
