use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	micro::run()?;
	Ok(())
}
