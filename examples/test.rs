use fontdue::layout::{HorizontalAlign, VerticalAlign};
use glam::UVec2;
use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, LayoutSettings, Text, TextFragment},
		DrawParams,
	},
	window::WindowMode,
	Context, ContextSettings, State,
};

struct MainState {
	normal_font: Font,
	bold_font: Font,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			normal_font: Font::from_file(
				ctx,
				"examples/Roboto-Regular.ttf",
				FontSettings::default(),
			)
			.unwrap(),
			bold_font: Font::from_file(ctx, "examples/Roboto-Bold.ttf", FontSettings::default())
				.unwrap(),
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		Text::with_multiple_fonts(
			ctx,
			&[&self.normal_font, &self.bold_font],
			&[
				TextFragment {
					font_index: 0,
					text: "This is some cool text,",
				},
				TextFragment {
					font_index: 1,
					text: "\nand some of it has emphasis.",
				},
				TextFragment {
					font_index: 0,
					text: "\nand some of it has emphasis.",
				},
				TextFragment {
					font_index: 0,
					text: "\nand some of it has emphasis.",
				},
			],
			LayoutSettings {
				max_width: Some(ctx.window_size().x as f32),
				max_height: Some(ctx.window_size().y as f32),
				horizontal_align: HorizontalAlign::Center,
				vertical_align: VerticalAlign::Middle,
				line_height: 4.0,
				..Default::default()
			},
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
