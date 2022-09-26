use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::ShapeStyle,
		texture::{Texture, TextureSettings},
	},
	ui::{
		align::Align,
		circle::Circle,
		image::Image,
		list::{List, Mode},
		Constraints, Widget,
	},
	Context, ContextSettings, State,
};

struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			texture: Texture::from_file(ctx, "examples/player.png", TextureSettings::default())
				.unwrap(),
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		Align::center(
			List::horizontal()
				.with_mode(Mode::SpaceEvenly)
				.with_child(Circle::new(50.0, ShapeStyle::Fill))
				.with_child(Circle::new(50.0, ShapeStyle::Fill))
				.with_child(Circle::new(200.0, ShapeStyle::Fill)),
		)
		.build(
			ctx,
			Constraints {
				min_size: Vec2::ZERO,
				max_size: ctx.window_size().as_vec2(),
			},
		)
		.draw(ctx);
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
