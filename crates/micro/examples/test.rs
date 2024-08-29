use std::{error::Error, time::Duration};

use fontdue::layout::{HorizontalAlign, VerticalAlign};
use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		mesh::Mesh,
		text::{Font, FontSettings, LayoutSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		Align, AxisSizing, Ellipse, Image, Mask, Padding, Polygon, Polyline, Rectangle, Stack,
		StackSettings, TextSettings, TextSizing, TextWidget, Ui, WidgetMouseEventChannel,
	},
	App, Context, ContextSettings,
};
use palette::{Darken, LinSrgb, LinSrgba};

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	font: Font,
	texture: Texture,
	ui: Ui,
	widget_mouse_event_channels: Vec<WidgetMouseEventChannel>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			font: Font::from_file(
				ctx,
				"resources/NotoSans-Regular.ttf",
				FontSettings::default(),
			)?,
			texture: Texture::from_file(
				ctx,
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
			ui: Ui::new(),
			widget_mouse_event_channels: vec![
				WidgetMouseEventChannel::new(),
				WidgetMouseEventChannel::new(),
				WidgetMouseEventChannel::new(),
			],
		})
	}
}

impl App<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		self.ui.render(
			ctx,
			ctx.window_size().as_vec2(),
			Stack::vertical(StackSettings {
				gap: 0.0,
				cross_align: 0.5,
				cross_sizing: AxisSizing::Shrink,
			})
			.with_child(
				Padding::all(10.0).with_child(
					Rectangle::new()
						.with_stroke(2.0, LinSrgb::WHITE)
						.with_fractional_size((0.5, 0.5)),
				),
			)
			.with_child(TextWidget::new(
				&self.font,
				"Hello, world!",
				TextSettings {
					sizing: TextSizing::Max {
						horizontal_align: HorizontalAlign::Left,
						vertical_align: VerticalAlign::Middle,
					},
					..Default::default()
				},
			)),
		)?;
		for (i, channel) in self.widget_mouse_event_channels.iter().enumerate() {
			while let Some(event) = channel.pop() {
				println!("{:?} ({})", event, i);
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiEvent {
	Click(usize),
	Hover(usize),
	Unhover(usize),
}
