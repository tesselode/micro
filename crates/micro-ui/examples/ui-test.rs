use micro::{
	App, ContextSettings,
	color::{ColorConstants, LinSrgba},
};
use micro_ui::{
	AxisSizing, Padding, Rectangle, RenderUiSettings, Sizing, Stack, StackSettings, Ui,
};

fn main() {
	micro::run(ContextSettings::default(), Test::new);
}

struct Test {
	ui: Ui,
}

impl Test {
	fn new() -> Self {
		Self { ui: Ui::new() }
	}
}

impl App for Test {
	fn draw(&mut self) {
		self.ui.render(
			RenderUiSettings::default(),
			Padding::all(10.0).child(
				Stack::horizontal(StackSettings {
					gap: 10.0,
					cross_align: 0.5,
					cross_sizing: AxisSizing::Shrink,
				})
				.children([
					Rectangle::new().fill(LinSrgba::RED).max_size((100.0, 50.0)),
					Rectangle::new().fill(LinSrgba::RED).max_size((50.0, 100.0)),
				]),
			),
		);
	}
}
