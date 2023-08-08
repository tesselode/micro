#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

out vec2 TexCoord;
out vec4 Color;

uniform mat3 globalTransform;
uniform mat3 localTransform;

void main()
{
    gl_Position = vec4(globalTransform * localTransform * vec3(aPos, 1.0), 1.0);
    TexCoord = aTexCoord;
    Color = aColor;
}
