/// @VERTEX
#version 460

layout (location = 0) in vec2 vertexInPos;
layout (location = 1) in vec2 vertexInUv;
layout (location = 2) in vec4 vertexInColor;

layout (binding = 0)
uniform DrawParams {
	mat4 globalTransform;
	mat4 localTransform;
	vec4 blendColor;
};

layout (location = 0) out vec2 vertexOutUv;
layout (location = 1) out vec4 vertexOutColor;

void main() {
	gl_Position = globalTransform * vec4(vertexInPos, 0.0, 1.0);
	vertexOutUv = vertexInUv;
	vertexOutColor = vertexInColor * blendColor;
}

/// @FRAGMENT
#version 460

layout (location = 0) in vec2 vertexOutUv;
layout (location = 1) in vec4 vertexOutColor;

layout (binding = 1) uniform texture2D inTexture;
layout (binding = 2) uniform sampler inSampler;

out vec4 FragColor;

void main() {
	FragColor = texture(sampler2D(inTexture, inSampler), vertexOutUv) * vertexOutColor;
}
