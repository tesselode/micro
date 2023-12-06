#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec4 aColor;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoord;
out vec4 Color;

uniform mat4 globalTransform;
uniform mat4 localTransform;
uniform mat4 normalTransform;

void main()
{
    gl_Position = globalTransform * localTransform * vec4(aPos, 1.0);
    FragPos = vec3(localTransform * vec4(aPos, 1.0));
    Normal = mat3(normalTransform) * aNormal;
    TexCoord = aTexCoord;
    Color = aColor;
}
