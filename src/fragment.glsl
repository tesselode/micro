#version 330 core
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
