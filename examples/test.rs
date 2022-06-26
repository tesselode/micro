use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, Text},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	font: Font,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			font: Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default())
				.unwrap(),
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		Text::new(ctx, &self.font, "hello!").draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
