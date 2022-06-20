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
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		let font =
			Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default()).unwrap();
		let text = Text::new(ctx, &font, "hello, world!");
		Self { font, text }
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		self.text.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
