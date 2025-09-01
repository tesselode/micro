use micro2::{App, Context, ContextSettings, Event, color::ColorConstants, input::Scancode};
use palette::LinSrgb;

fn main() {
	micro2::run(ContextSettings::default(), |_| Test);
}

struct Test;

impl App for Test {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}

		if let Event::KeyPressed {
			key: Scancode::Return,
			..
		} = event
		{
			ctx.set_clear_color(LinSrgb::BLUE);
		}
	}
}
