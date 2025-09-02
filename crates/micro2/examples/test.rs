use micro2::{
	App, Context, ContextSettings, Event,
	graphics::text::{Font, FontSettings, LayoutSettings, Text},
	input::Scancode,
};

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {
	text: Text,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		let font = Font::from_file(
			ctx,
			"resources/NotoSans-Regular.ttf",
			FontSettings::default(),
		)
		.unwrap();
		let text = Text::new(ctx, &font, "Hello, world!", LayoutSettings::default());
		Self { text }
	}
}

impl App for Test {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		self.text.draw(ctx);
	}
}
