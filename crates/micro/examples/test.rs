use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle, Vertex},
		sprite_batch::{SpriteBatch, SpriteId},
		texture::Texture,
		DrawParams,
	},
	input::Scancode,
	math::Rect,
	window::WindowMode,
	Context, ContextSettings, Event, State,
};

const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

struct MainState {
	texture: Texture,
	sprite_batch: SpriteBatch,
	sprite_id: SpriteId,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::from_file(ctx, "crates/micro/examples/tree.png")?;
		let mut sprite_batch = SpriteBatch::new(ctx, &texture, 3);
		let sprite_id = sprite_batch.add_region(
			ctx,
			Rect::xywh(10.0, 10.0, 40.0, 60.0),
			Vec2::new(50.0, 100.0),
		)?;
		Ok(Self {
			texture,
			sprite_batch,
			sprite_id,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed(Scancode::Space) = event {
			self.sprite_batch.remove(ctx, self.sprite_id)?;
			self.sprite_batch.add_region(
				ctx,
				Rect::xywh(10.0, 10.0, 40.0, 60.0),
				Vec2::new(200.0, 300.0),
			)?;
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.sprite_batch.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			},
			..Default::default()
		},
		MainState::new,
	)
}
