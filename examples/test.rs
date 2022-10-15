use glam::UVec2;
use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	input::Scancode,
	window::WindowMode,
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
		if let Event::KeyPressed(Scancode::F1) = event {
			ctx.set_window_mode(match ctx.window_mode() {
				WindowMode::Fullscreen => WindowMode::Windowed {
					size: UVec2::new(500, 500),
				},
				WindowMode::Windowed { .. } => WindowMode::Fullscreen,
			})
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		let window_size = ctx.window_size();
		Text::new(
			ctx,
			&self.font,
			&format!("{}x{}", window_size.x, window_size.y),
			LayoutSettings::default(),
		)
		.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}
