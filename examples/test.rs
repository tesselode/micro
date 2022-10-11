use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	input::Scancode,
	Context, ContextSettings, Event, State,
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
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyDown {
			scancode: Some(Scancode::F4),
			..
		} = event
		{
			ctx.set_fullscreen(!ctx.fullscreen());
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		let window_size = ctx.window_size();
		let monitor_resolution = ctx.monitor_resolution();
		Text::new(
			ctx,
			&self.font,
			&format!("{}x{}", ctx.mouse_position().x, ctx.mouse_position().y),
			LayoutSettings::default(),
		)
		.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}
