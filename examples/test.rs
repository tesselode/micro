use micro::{
	graphics::{
		color::Rgba,
		sprite_batch::{Sprite, SpriteBatch},
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};

struct MainState {
	sprite_batch: SpriteBatch,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		let texture =
			Texture::from_file(ctx, "examples/player.png", TextureSettings::default()).unwrap();
		let mut sprite_batch = SpriteBatch::new(ctx, &texture, 1);
		sprite_batch
			.add(Sprite {
				display_rect: Rect::xywh(0.0, 0.0, 50.0, 50.0),
				texture_rect: Rect::xywh(0.0, 0.0, 50.0, 50.0),
			})
			.unwrap();
		Self { sprite_batch }
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		self.sprite_batch.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
