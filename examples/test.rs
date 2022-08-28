use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, Text},
		DrawParams,
	},
	input::Scancode,
	Context, ContextSettings, Event, State,
};

struct MainState {
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		let font =
			Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default()).unwrap();
		let text = Text::new(ctx, &font, "hello world!");
		println!("{}", text.num_glyphs());
		Self { text }
	}
}

impl State for MainState {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyDown {
			scancode: Some(Scancode::Escape),
			..
		} = event
		{
			ctx.quit();
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		self.text.draw_range(ctx, 3.., DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
