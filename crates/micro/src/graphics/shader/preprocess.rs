use super::{Shader, DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER};

const VERSION_STRING: &str = "#version 330 core\n";

impl Shader {
	pub(super) fn preprocess_shader_code(code: &str) -> String {
		VERSION_STRING.to_owned() + code
	}

	pub(super) fn split_combined(combined: &str) -> SplitShaderCode {
		let mut combined = combined.to_owned();
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
		let vertex = "#define VERTEX\n".to_owned() + &combined;
		let fragment = "#define FRAGMENT\n".to_owned() + &combined;
		SplitShaderCode { vertex, fragment }
	}
}

pub(super) struct SplitShaderCode {
	pub vertex: String,
	pub fragment: String,
}
