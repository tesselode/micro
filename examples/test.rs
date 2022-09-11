use fontdue::layout::{HorizontalAlign, VerticalAlign};
use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		let font = Font::from_file(
			ctx,
			"examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 20.0,
				..Default::default()
			},
		)
		.unwrap();
		Self {
			text: Text::new(
				ctx,
				&font,
				"This is some test text.\nHopefully the layout looks good!\n\nExtra line break",
				LayoutSettings {
					max_width: Some(ctx.window_size().x as f32),
					max_height: Some(ctx.window_size().y as f32),
					horizontal_align: HorizontalAlign::Center,
					vertical_align: VerticalAlign::Middle,
					..Default::default()
				},
			),
		}
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
