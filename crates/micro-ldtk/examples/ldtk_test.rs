use std::error::Error;

use micro_ldtk::Level;

fn main() -> Result<(), Box<dyn Error>> {
	let level_string = std::fs::read_to_string("resources/Test.ldtkl")?;
	let level = serde_json::from_str::<Level>(&level_string)?;
	dbg!(level);
	Ok(())
}
