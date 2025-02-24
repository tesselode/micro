use std::error::Error;

use micro_wgpu::{App, ContextSettings};

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(ContextSettings::default(), |_| Ok(Test))
}

struct Test;

impl App for Test {
	type Error = Box<dyn Error>;
}
