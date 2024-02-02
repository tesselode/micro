use super::Shader;

const VERSION_STRING: &str = "#version 330 core\n";

impl Shader {
	pub(super) fn preprocess_shader_code(code: &str) -> String {
		VERSION_STRING.to_owned() + code
	}

	pub(super) fn split_combined(combined: &str) -> SplitShaderCode {
		let vertex = "#define VERTEX\n".to_owned() + combined;
		let fragment = "#define FRAGMENT\n".to_owned() + combined;
		SplitShaderCode { vertex, fragment }
	}
}

pub(super) struct SplitShaderCode {
	pub vertex: String,
	pub fragment: String,
}
