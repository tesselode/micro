#version 460

#ifdef VERTEX

layout (location = 0) in vec2 aPos;

layout (binding = 0)
uniform DrawParams {
	mat4 globalTransform;
	mat4 localTransform;
	vec4 blendColor;
};

layout (location = 0) out vec2 texCoord;
layout (location = 1) out vec4 vertexColor;

void main()
{
    gl_Position = globalTransform * vec4(aPos, 0.0, 1.0);
    texCoord = vec2(0.0, 0.0);
    vertexColor = blendColor;
}

#endif

#ifdef FRAGMENT

layout (location = 0) in vec2 texCoord;
layout (location = 1) in vec4 vertexColor;

layout (binding = 1) uniform texture2D inTexture;
layout (binding = 2) uniform sampler inSampler;

out vec4 fragColor;

void main()
{
    fragColor = texture(sampler2D(inTexture, inSampler), texCoord) * vertexColor;
}

#endif