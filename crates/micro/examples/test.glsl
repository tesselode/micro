#ifdef VERTEX

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

out vec2 TexCoord;
out vec4 Color;

uniform mat4 globalTransform;
uniform mat4 localTransform;

#include "test.glsl.fragment"

#endif

#ifdef FRAGMENT

out vec4 FragColor;

in vec2 TexCoord;
in vec4 Color;

uniform sampler2D ourTexture;
uniform vec4 blendColor;

void main()
{
    FragColor = texture(ourTexture, TexCoord) * Color * blendColor;
    if (FragColor.a == 0.0) {
        discard;
    }
}

#endif
