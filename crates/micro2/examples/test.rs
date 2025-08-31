use micro2::{App, Context, ContextSettings, Event, input::Scancode};

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
	}
}
