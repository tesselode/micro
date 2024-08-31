use std::error::Error;

use micro::{
	color::ColorConstants,
	graphics::Msaa,
	ui::{Align, Rectangle, Transform, Ui, WidgetMouseEventChannel},
	App, Context, ContextSettings,
};
use palette::LinSrgb;

fn main() {
	micro::run(ContextSettings::default(), Test::new);
}

struct Test {
	ui: Ui,
	widget_mouse_event_channel: WidgetMouseEventChannel,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let max_msaa_level = ctx.max_msaa_level();
		println!("{:?}", max_msaa_level);
		dbg!(Msaa::levels_up_to(max_msaa_level).collect::<Vec<_>>());
		Ok(Self {
			ui: Ui::new(),
			widget_mouse_event_channel: WidgetMouseEventChannel::new(),
		})
	}
}

impl App<Box<dyn Error>> for Test {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		self.ui.render(
			ctx,
			ctx.window_size().as_vec2(),
			Align::center().with_child(
				Transform::rotation(1.0).with_child(
					Rectangle::new()
						.with_max_size((50.0, 100.0))
						.with_stroke(2.0, LinSrgb::WHITE)
						.with_mouse_event_channel(&self.widget_mouse_event_channel),
				),
			),
		)?;
		while let Some(event) = self.widget_mouse_event_channel.pop() {
			dbg!(event);
		}
		Ok(())
	}
}
