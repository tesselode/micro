use std::error::Error;

use micro_aseprite_exporter::export_aseprite_files;

fn main() -> Result<(), Box<dyn Error>> {
	let mut aseprite_file_paths = vec![];
	for entry in std::fs::read_dir("resources/ppl")? {
		let entry = entry?;
		aseprite_file_paths.push(entry.path());
	}
	export_aseprite_files(
		&aseprite_file_paths,
		"resources/test.png",
		"resources/animations",
		1024,
	)?;
	Ok(())
}
