use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::ShapeStyle,
		texture::{Texture, TextureSettings},
	},
	ui::{
		align::Align,
		constrained::Constrained,
		container::Container,
		ellipse::Ellipse,
		flex::Flex,
		image::Image,
		list::{List, Mode},
		rectangle::Rectangle,
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
		Container::new()
			.with_child(
				Vec2::new(100.0, 50.0),
				Constrained::new(
					Constraints::max_only(Vec2::new(100.0, 100.0)),
					Rectangle::new(ShapeStyle::Fill, Rgba::RED),
				),
			)
			.with_child(
				Vec2::new(150.0, 100.0),
				Constrained::new(
					Constraints::max_only(Vec2::new(100.0, 100.0)),
					Rectangle::new(ShapeStyle::Fill, Rgba::BLUE),
				),
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
