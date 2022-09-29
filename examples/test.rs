use fontdue::layout::HorizontalAlign;
use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::ShapeStyle,
		text::{Font, FontSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		aligned::Aligned,
		constrained::Constrained,
		container::Container,
		ellipse::Ellipse,
		flex::Flex,
		image::Image,
		list::{List, Mode},
		padded::Padded,
		rectangle::Rectangle,
		text::Text,
		transformed::Transformed,
		Widget,
	},
	Context, ContextSettings, State,
};

struct MainState {
	texture: Texture,
	font: Font,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			texture: Texture::from_file(ctx, "examples/player.png", TextureSettings::default())
				.unwrap(),
			font: Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default())
				.unwrap(),
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		List::vertical()
			.with_child(
				Text::new(&self.font, "hello, world! long text")
					.with_horizontal_align(HorizontalAlign::Center),
			)
			.with_child(
				Text::new(&self.font, "second line of text")
					.with_horizontal_align(HorizontalAlign::Right),
			)
			.build(ctx, Vec2::new(100.0, 500.0))
			.draw(ctx);
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
