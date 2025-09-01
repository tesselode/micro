#version 460

#ifdef VERTEX

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

layout (set = 0, binding = 0)
uniform DrawParams {
	mat4 globalTransform;
	mat4 localTransform;
	vec4 blendColor;
};

layout (set = 1, binding = 0)
uniform ShaderParams {
    vec2 translate;
};

layout (location = 0) out vec2 texCoord;
layout (location = 1) out vec4 vertexColor;

void main()
{
    gl_Position = globalTransform * vec4(aPos + translate, 0.0, 1.0);
    texCoord = aTexCoord;
    vertexColor = aColor * blendColor;
}

#endif

#ifdef FRAGMENT

layout (location = 0) in vec2 texCoord;
layout (location = 1) in vec4 vertexColor;

out vec4 fragColor;

void main()
{
    fragColor = vertexColor;
}

#endif