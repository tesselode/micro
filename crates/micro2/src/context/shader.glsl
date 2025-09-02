#version 460

#ifdef VERTEX

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

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
    texCoord = aTexCoord;
    vertexColor = aColor * blendColor;
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