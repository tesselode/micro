#version 330 core

// vertex attributes
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec4 aColor;

// instance attributes
layout (location = 4) in mat4 aLocalTransform;
layout (location = 8) in mat4 aNormalTransform;
layout (location = 12) in vec4 aBlendColor;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoord;
out vec4 Color;

uniform mat4 globalTransform;

void main()
{
    gl_Position = globalTransform * aLocalTransform * vec4(aPos, 1.0);
    FragPos = vec3(aLocalTransform * vec4(aPos, 1.0));
    Normal = mat3(aNormalTransform) * aNormal;
    TexCoord = aTexCoord;
    Color = aColor * aBlendColor;
}
