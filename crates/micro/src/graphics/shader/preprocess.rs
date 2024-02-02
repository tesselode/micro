use std::path::Path;

use once_cell::sync::Lazy;
use regex_lite::{Captures, Regex};

use super::{LoadShaderError, DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER};

static INCLUDE_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new("#include \"(.*)\"").expect("error compiling include regex"));
const VERSION_STRING: &str = "#version 330 core\n";

pub(crate) struct CombinedShaderCode(String);

impl CombinedShaderCode {
	pub fn from_file(path: &Path) -> std::io::Result<Self> {
		std::fs::read_to_string(path).map(|combined| Self::from_str(&combined))
	}

	pub fn from_str(s: &str) -> Self {
		let mut combined = s.to_owned();
		if !combined.contains("#ifdef VERTEX") {
			combined += "\n#ifdef VERTEX\n";
			combined += DEFAULT_VERTEX_SHADER;
			combined += "\n#endif\n";
		}
		if !combined.contains("#ifdef FRAGMENT") {
			combined += "\n#ifdef FRAGMENT\n";
			combined += DEFAULT_FRAGMENT_SHADER;
			combined += "\n#endif\n";
		}
		Self(combined)
	}

	pub fn split(&self) -> SplitShaderCode {
		let vertex = RawShaderCode("#define VERTEX\n".to_owned() + &self.0);
		let fragment = RawShaderCode("#define FRAGMENT\n".to_owned() + &self.0);
		SplitShaderCode { vertex, fragment }
	}
}

pub(crate) struct RawShaderCode(pub String);

impl RawShaderCode {
	pub fn from_file(path: &Path) -> std::io::Result<Self> {
		std::fs::read_to_string(path).map(Self)
	}

	pub fn preprocess(
		&self,
		path: Option<&Path>,
	) -> Result<PreprocessedShaderCode, LoadShaderError> {
		replace_fallible(
			&INCLUDE_REGEX,
			&self.0,
			|captures| -> Result<String, LoadShaderError> {
				let relative_path = &captures[1];
				let full_path = path
					.ok_or(LoadShaderError::IncludeFromStrError)?
					.parent()
					.expect("shader path has no parent")
					.join(relative_path);
				RawShaderCode::from_file(&full_path)?
					.preprocess(Some(&full_path))
					.map(|code| code.0)
			},
		)
		.map(PreprocessedShaderCode)
	}
}

pub(crate) struct PreprocessedShaderCode(String);

pub(crate) struct VersionedShaderCode(pub String);

impl From<PreprocessedShaderCode> for VersionedShaderCode {
	fn from(PreprocessedShaderCode(preprocessed): PreprocessedShaderCode) -> Self {
		Self(VERSION_STRING.to_owned() + &preprocessed)
	}
}

pub(crate) struct SplitShaderCode {
	pub vertex: RawShaderCode,
	pub fragment: RawShaderCode,
}

fn replace_fallible<E>(
	re: &Regex,
	haystack: &str,
	replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
	let mut new = String::with_capacity(haystack.len());
	let mut last_match = 0;
	for caps in re.captures_iter(haystack) {
		let m = caps.get(0).unwrap();
		new.push_str(&haystack[last_match..m.start()]);
		new.push_str(&replacement(&caps)?);
		last_match = m.end();
	}
	new.push_str(&haystack[last_match..]);
	Ok(new)
}
