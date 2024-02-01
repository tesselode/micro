use itertools::Itertools;
use once_cell::sync::Lazy;
use regex_lite::Regex;

use super::Shader;

static VERSION_STRING_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new("#version .*").expect("could not compile version string regex"));

impl Shader {
	pub(super) fn split_combined(combined: &str) -> SplitShaderCode {
		let version_string = VERSION_STRING_REGEX
			.captures(combined)
			.and_then(|captures| captures.get(0))
			.map(|m| m.as_str())
			.unwrap_or_default();
		let main_code = VERSION_STRING_REGEX.split(combined).join("");
		let vertex = version_string.to_owned() + "#define VERTEX" + &main_code;
		let fragment = version_string.to_owned() + "#define FRAGMENT" + &main_code;
		SplitShaderCode { vertex, fragment }
	}
}

pub(super) struct SplitShaderCode {
	pub vertex: String,
	pub fragment: String,
}
